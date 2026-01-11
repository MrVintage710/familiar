use mlua::{IntoLua, Lua, MetaMethod, UserData, Value, Variadic};
use pak_db::value::PakValue;
use serde::{Deserialize, Serialize};

use crate::stat::{derive::{StatDerive, StatDeriveLink}, field::{StatBlockField, StatSourceProvider}, value::StatValue};

//==============================================================================================
//        StatSource
//==============================================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatSource {
    value : StatValue,
    derives : Vec<StatDeriveLink>
}

impl StatSource {
    pub fn new(value : impl Into<StatValue>) -> Self {
        StatSource { value : value.into(), derives: Default::default() }
    }
    
    pub fn set(&mut self, lua : &Lua, value : impl Into<StatValue>) {
        let value = value.into();
        for link in self.derives.iter() {
            link.send(lua, value.clone());
        }
        self.value = value;
    }
    
    pub fn has_derivative(&self, addr : usize) -> bool {
        self.derives.iter().any(|link| link.get_pointer_usize() == addr)
    }
    
    pub fn derivative_count(&self) -> usize {
        self.derives.len()
    }
    
    pub(crate) fn links(&self) -> Vec<(usize, usize)> {
        self.derives.iter().map(|link| (link.get_index(), link.get_pointer_usize())).collect()
    }
    
    pub(crate) fn bind(&mut self, derivative : &mut StatBlockField, addr : usize) {
        let Some(link) = self.derives.iter_mut().find(|link| link.get_pointer_usize() == addr) else { return };
        let Some(derive) = derivative.as_derive_mut() else { return };
        link.revalidate(derive.weak());
    }
    
    pub fn as_pak_value(&self) -> PakValue {
        self.get().into()
    }
}

impl StatSourceProvider for StatSource {
    fn get(&self) -> StatValue {
        self.value.clone()
    }
    
    fn push_derive(&mut self, derive : &StatDerive) {
        self.derives.push(derive.bind_link(self.get()));
    }
}

impl UserData for StatSource {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |_lua, this, ()| {
            Ok(this.get())
        });
        
        methods.add_method_mut("set", |lua, this, value : StatValue| {
            this.set(lua, value);
            Ok(())
        });
        
        methods.add_meta_method(MetaMethod::Eq, |lua, this, other : Value| {
            Ok(this.get().into_lua(lua)? == other)
        });
    }
}