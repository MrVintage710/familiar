use std::{fmt::{Debug, DebugStruct}, sync::{Arc, RwLock, Weak}};

use mlua::{FromLuaMulti, Function, IntoLua, IntoLuaMulti, Lua, MaybeSend, MultiValue, UserData};
use pak_db::value::PakValue;
use serde::{Deserialize, Serialize, de::{self, Visitor}, ser::SerializeSeq};

use crate::stat::{field::{StatBlockField, StatProviderRef, StatSourceProvider}, source::StatSource, value::StatValue};

//==============================================================================================
//        StatDerive
//==============================================================================================

pub struct StatDerive {
    core : Arc<RwLock<StatDeriveCore>>,
}

impl StatDerive {
    pub fn new<C, I, F>(lua : &Lua, mut providers : Vec<&mut dyn StatSourceProvider>, callback : C) -> mlua::Result<Self> 
    where C: Fn(&Lua, F) -> mlua::Result<I> + MaybeSend + 'static, I : IntoLuaMulti, F : FromLuaMulti {
        let callback = lua.create_function(callback)?;
        let values = providers.iter().map(|provider| provider.get()).collect::<Vec<_>>();
        let core = StatDeriveCore::new(lua, callback, values)?;
        let derive = StatDerive { core: Arc::new(RwLock::new(core)) };
        for provider in providers.iter_mut() {
            provider.push_derive(&derive);
        }
        Ok(derive)
    }
    
    pub(crate) fn new_lua_ref(lua : &Lua, mut providers : Vec<StatProviderRef>, callback : Function) -> mlua::Result<Self> {
        let values = providers.iter().map(|provider| provider.get()).collect::<Vec<_>>();
        let core = StatDeriveCore::new(lua, callback, values)?;
        let mut derive = StatDerive { core: Arc::new(RwLock::new(core)) };
        for provider in providers.iter_mut() {
            if provider.is_constant() { derive.init_constant(provider.get()); }
            provider.push_derive(&derive);
        }
        Ok(derive)
    }
    
    pub(crate) fn new_lua(lua : &Lua, mut providers : Vec<StatBlockField>, callback : Function) -> mlua::Result<Self> {
        let values = providers.iter().map(|provider| provider.get()).collect::<Vec<_>>();
        let core = StatDeriveCore::new(lua, callback, values)?;
        let derive = StatDerive { core: Arc::new(RwLock::new(core)) };
        for provider in providers.iter_mut() {
            provider.push_derive(&derive);
        }
        Ok(derive)
    }
    
    pub(crate) fn new_lua_with_values(lua : &Lua, mut providers : Vec<StatBlockField>, callback : Function, values : Vec<StatValue>) -> mlua::Result<Self> {
        let core = StatDeriveCore::new(lua, callback, values)?;
        let mut derive = StatDerive { core: Arc::new(RwLock::new(core)) };
        for provider in providers.iter_mut() {
            if provider.is_constant() { derive.init_constant(provider.get()); }
            provider.push_derive(&derive);
        }
        Ok(derive)
    }
    
    pub(crate) fn bind_link(&self, value : StatValue) -> StatDeriveLink {
        let index = self.core.write().unwrap().init_param(value);
        StatDeriveLink::Valid { link: Arc::downgrade(&self.core), index }
    }
    
    pub(crate) fn get_pointer_usize(&self) -> usize {
        Arc::as_ptr(&self.core) as usize
    }
    
    pub(crate) fn derivative_count(&self) -> usize {
        self.core.read().unwrap().source.derivative_count()
    }
    
    pub fn has_derivative(&self, addr : usize) -> bool {
        self.core.read().unwrap().source.has_derivative(addr)
    }
    
    fn init_constant(&mut self, value : StatValue) {
        self.core.write().unwrap().init_param(value);
    }
    
    pub(crate) fn core(&self) -> std::sync::RwLockReadGuard<'_, StatDeriveCore> {
        self.core.read().unwrap()
    }
    
    pub(crate) fn core_mut(&self) -> std::sync::RwLockWriteGuard<'_, StatDeriveCore> {
        self.core.write().unwrap()
    }
    
    pub(crate) fn links(&self) -> Vec<(usize, usize)> {
        self.core.read().unwrap().source.links()
    }
    
    pub(crate) fn weak(&self) -> Weak<RwLock<StatDeriveCore>> {
        Arc::downgrade(&self.core)
    }
    
    pub(crate) fn bind(&mut self, derivative : &mut StatBlockField, addr : usize) {
        self.core_mut().source.bind(derivative, addr);
    }
    
    pub fn as_pak_value(&self) -> PakValue {
        self.core().source.as_pak_value()
    }
}

impl Debug for StatDerive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Derive");
        self.core().fmt(&mut s);
        s.field("addr", &self.get_pointer_usize()).finish()
    }
}

impl Serialize for StatDerive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.core.read().unwrap().serialize(serializer)
    }
}

impl <'de> Deserialize<'de> for StatDerive {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        let core = StatDeriveCore::deserialize(deserializer)?;
        Ok(StatDerive { core: Arc::new(RwLock::new(core)) })
    }
}

impl Clone for StatDerive {
    fn clone(&self) -> Self {
        Self { core: Arc::new(RwLock::new(self.core().clone())) }
    }
}

impl StatSourceProvider for StatDerive {
    fn get(&self) -> StatValue {
        self.core.read().unwrap().source.get()
    }

    fn push_derive(&mut self, derive : &StatDerive) {
        self.core.write().unwrap().source.push_derive(derive);
    }
}

impl UserData for StatDerive {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |_lua, this, ()| {
            Ok(this.get())
        });
    }
}

//==============================================================================================
//        StatDerive Link
//==============================================================================================

pub(crate) enum StatDeriveLink {
    Valid {
        link : Weak<RwLock<StatDeriveCore>>,
        index : usize
    },
    Invalid {
        link : usize,
        index : usize
    }
}

impl StatDeriveLink {
    pub fn send(&self, lua : &Lua, value : StatValue) {
        let StatDeriveLink::Valid { link, index } = self else { return };
        let Some(core) = link.upgrade() else { println!("Broken Link!"); return };
        let Ok(mut core) = core.write() else { return };
        core.send(lua, value, *index);
    }
    
    pub(crate) fn get_pointer_usize(&self) -> usize {
        match self {
            StatDeriveLink::Valid { link, .. } => link.as_ptr() as usize,
            StatDeriveLink::Invalid { link, .. } => *link,
        }    
    }
    
    pub(crate) fn get_index(&self) -> usize {
        match self {
            StatDeriveLink::Valid { index, .. } => *index,
            StatDeriveLink::Invalid { index, .. } => *index,
        }
    }
    
    pub(crate) fn revalidate(&mut self, link : Weak<RwLock<StatDeriveCore>>) {
        *self = StatDeriveLink::Valid { link, index: self.get_index() };
    }
}

impl Debug for StatDeriveLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-({})> {}", self.get_index(), self.get_pointer_usize())
    }
}

impl Serialize for StatDeriveLink {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut s = serializer.serialize_seq(Some(2))?;
        s.serialize_element(&self.get_pointer_usize())?;
        s.serialize_element(&self.get_index())?;
        s.end()
    }
}

impl <'de> Deserialize<'de> for StatDeriveLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        struct StatDeriveLinkVistor;
        
        impl <'de> Visitor<'de> for StatDeriveLinkVistor {
            type Value = StatDeriveLink;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a StatDeriveLink")
            }
            
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: serde::de::SeqAccess<'de>, {
                let link = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let index = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(StatDeriveLink::Invalid { link, index })
            }
        }
        
        deserializer.deserialize_seq(StatDeriveLinkVistor)
    }
}

impl Clone for StatDeriveLink {
    fn clone(&self) -> Self {
        match self {
            Self::Valid { link, index } => Self::Invalid { link: link.as_ptr() as usize, index: *index },
            Self::Invalid { link, index } => Self::Invalid { link: link.clone(), index: index.clone() },
        }
    }
}

//==============================================================================================
//        StatDeriveCore
//==============================================================================================

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct StatDeriveCore {
    source : StatSource,
    callback : Vec<u8>,
    params : Vec<StatValue>,
    current_param : usize
}

impl StatDeriveCore {
    fn new(lua : &Lua, callback : Function, values : Vec<StatValue>) -> mlua::Result<Self> {
        
        let lua_values = values.iter().cloned().map(|value| value.into_lua(lua)).filter_map(Result::ok).collect::<Vec<_>>();
        let value : StatValue = callback.call(MultiValue::from_vec(lua_values))?;
        let source = StatSource::new(value);
        Ok(StatDeriveCore { source, callback : callback.dump(true), params: values, current_param : 0 })
    }
    
    fn send(&mut self, lua : &Lua, value : StatValue, index : usize) {
        *self.params.get_mut(index).unwrap() = value;
        self.update(lua);
    }
    
    fn update(&mut self, lua : &Lua) {
        let values = self.params.clone().into_iter().map(|value| value.into_lua(&lua)).filter_map(Result::ok).collect::<Vec<_>>();
        let callback = lua.load(self.callback.clone()).into_function().unwrap();
        let value : StatValue = callback.call(MultiValue::from_vec(values)).unwrap();
        self.source.set(lua, value);
    }
    
    fn init_param(&mut self, value : StatValue) -> usize {
        let index = self.current_param;
        let v = self.params.get_mut(index).unwrap();
        *v = value;
        self.current_param += 1;
        index
    }
    
    fn fmt(&self, f: &mut DebugStruct<'_, '_>) {
        f.field("source", &self.source).field("params", &self.params);
    }
}