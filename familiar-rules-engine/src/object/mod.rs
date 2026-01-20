use std::{collections::{VecDeque}, sync::{Arc, RwLock, Weak}, vec};

use mlua::{FromLua, IntoLua, Lua, MetaMethod, UserData, Value, Variadic};
use pak_db::index::{Indices, PakSearchable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{action::definition::ActionDef, asset::Asset, common::{HasItemMeta, ItemMeta, enable_meta_methods_for_ref}, error::FreResult, feature::Feature, lua::reference::LuaRef, stat::{field::{StatBlockField, StatSourceProvider}, statblock::{StatBlock, StatBlockPath}, value::StatValue}};

//==============================================================================================
//        Object
//==============================================================================================

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Object {
    meta : ItemMeta,
    statblock : StatBlock,
    applied_statblock : StatBlock,
    features : VecDeque<Uuid>,
    // actions : Vec<Action>,
    assets : Vec<Uuid>
}

impl Object {
    pub fn new(name : &str, stats : StatBlock) -> Object {
        Object { meta: ItemMeta::new(name, "Object"), statblock: stats.clone(), applied_statblock : stats, features: VecDeque::new(), assets: vec![] }
    }
    
    pub fn add_features(&mut self, features : VecDeque<&Feature>) {
        let mut ids = features.iter().map(|feature| feature.get_meta().uuid()).collect::<VecDeque<_>>();
        self.features.append(&mut ids);
    }
    
    pub fn add_assets(&mut self, assets : Vec<&Asset>) {
        let mut ids = assets.iter().map(|asset| asset.get_meta().uuid()).collect::<Vec<_>>();
        self.assets.append(&mut ids);
    }
    
    pub fn stats(&self) -> &StatBlock {
        &self.statblock
    }
    
    pub fn stats_mut(&mut self) -> &mut StatBlock {
        &mut self.statblock
    }

    pub fn assets(&self) -> &[Uuid] {
        &self.assets
    }
    
    // pub fn apply(&self, lua : &Lua) -> VreResult<StatBlock> {
    //     let result = Arc::new(RwLock::new(self.statblock.clone()));
    //     let mut errors = Vec::<VreError>::new();
    //     for feature in self.features.iter() {
    //         let lua_stats = LuaStatBlockRef::from_arc(&result);
    //         let Some(callback) = feature.get_apply_function(lua) else { continue };
    //         let error = callback.call::<()>(lua_stats);
    //         if let Err(err) = error { errors.push(err.into());}
    //     }

    //     if errors.is_empty() {
    //         Ok(Arc::into_inner(result).unwrap().into_inner().unwrap())
    //     } else {
    //         Err(VreError::MultipleErrors(errors))
    //     }
    // }
}

impl HasItemMeta for Object {
    fn get_meta(&self) -> &ItemMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut ItemMeta {
        &mut self.meta
    }
}


impl IntoLua for Object {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        LuaObject(Arc::new(RwLock::new(self))).into_lua(lua)
    }
}


impl FromLua for Object {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let Some(userdata) = value.as_userdata() else { return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Object".to_string(), message: None }) };
        let Ok(object) = userdata.take::<LuaObject>() else { return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Object".to_string(), message: None }) };
        let object = Arc::into_inner(object.0).unwrap().into_inner().unwrap();
        Ok(object)
    }
}

impl LuaRef for Object {
    type RefType = LuaObjectRef;
    
    fn make_ref(this : &Arc<RwLock<Self>>) -> Self::RefType {
        LuaObjectRef(Arc::downgrade(this), String::new())
    }

    fn from_ref(reference : &Self::RefType) -> Option<Arc<RwLock<Self>>> {
        reference.0.upgrade()
    }
}

impl PakSearchable for Object {
    fn get_indices(&self, indices : &mut Indices) {
        self.meta.get_indices(indices);
        self.applied_statblock.get_indices(indices);
    }
}

//==============================================================================================
//        Lua Object Common Functions
//==============================================================================================

fn lua_object_index(lua : &Lua, reference : &Arc<RwLock<Object>>, root : &str, key : &str) -> mlua::Result<Value> {
    let Ok(object) = reference.read() else {return Ok(Value::Nil) };
    let path = root.append_path(key);
    let Some(field) = object.applied_statblock.get_field(&path) else {return Ok(Value::Nil)};
    match field {
        StatBlockField::Stat(_) => LuaObjectStatRef(Arc::downgrade(reference), path).into_lua(lua),
        StatBlockField::Derive(_) => LuaObjectDeriveRef(Arc::downgrade(reference), path).into_lua(lua),
        StatBlockField::Constant(stat_value) => stat_value.clone().into_lua(lua),
        StatBlockField::Table(_) => LuaObjectRef(Arc::downgrade(reference), path).into_lua(lua),
    }
}

fn lua_object_index_new(reference : &Arc<RwLock<Object>>, root : &str, key : &str, value : StatBlockField) -> mlua::Result<()> {
    let Ok(mut object) = reference.write() else {return Ok(()) };
    let index = root.append_path(key);
    object.applied_statblock.set_field(&index, value, true);
    Ok(())
}

fn lua_object_add_features(reference : &Arc<RwLock<Object>>, features : Variadic<Feature>) -> mlua::Result<()> {
    let Ok(mut object) = reference.write() else { return Ok(()) };
    object.add_features(features.iter().collect());
    Ok(())
}

fn lua_object_add_assets(reference : &Arc<RwLock<Object>>, assets : Variadic<Asset>) -> mlua::Result<()> {
    let Ok(mut object) = reference.write() else { return Ok(()) };
    object.add_assets(assets.iter().collect());
    Ok(())
}

fn lua_provider_get(lua : &Lua, reference : &Arc<RwLock<Object>>, root : &str) -> mlua::Result<Value> {
    let Ok(object) = reference.read() else { return Ok(Value::Nil) };
    let Some(field) = object.applied_statblock.get_field(root) else { return Ok(Value::Nil) };
    field.get().into_lua(lua)
}

fn lua_stat_set(lua : &Lua, reference : &Arc<RwLock<Object>>, root : &str, value : StatValue) -> mlua::Result<()> {
    let Ok(mut object) = reference.write() else { return Ok(()) };
    let Some(field) = object.applied_statblock.get_field_mut(root) else { return Ok(()) };
    field.set(lua, value);
    Ok(())
}

//==============================================================================================
//        LuaObject
//==============================================================================================

pub struct LuaObject(Arc<RwLock<Object>>);

impl UserData for LuaObject {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("add_features", |_, this, features : Variadic<Feature>| {
            lua_object_add_features(&this.0, features)
        });
        
        methods.add_method("add_assets", |_, this, assets : Variadic<Asset>| {
            lua_object_add_assets(&this.0, assets)
        });
        
        methods.add_meta_method(MetaMethod::NewIndex, |_, this, (key, value) : (String, StatBlockField)| {
            lua_object_index_new(&this.0, "", &key, value)
        });
        
        methods.add_meta_method(MetaMethod::Index, |lua, this, key : String| {
            lua_object_index(lua, &this.0, "", &key)
        });
    }
}

//==============================================================================================
//        LuaObjectRef
//==============================================================================================

#[derive(Clone)]
pub struct LuaObjectRef(Weak<RwLock<Object>>, String);

impl UserData for LuaObjectRef {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::NewIndex, |_lua, this, (key, value) : (String, StatBlockField)| {
            let Some(reference) = this.0.upgrade() else { return Ok(()) };
            lua_object_index_new( &reference, &this.1, &key, value)
        });
        
        methods.add_meta_method(MetaMethod::Index, |lua, this, key : String| {
            let Some(reference) = this.0.upgrade() else { return Ok(Value::Nil) };
            lua_object_index(lua, &reference, &this.1, &key)
        });
        
        methods.add_method("add_features", |_lua, this, features : Variadic<Feature>| {
            let Some(object) = this.0.upgrade() else { return Ok(()) };
            lua_object_add_features( &object, features)
        });
        
        methods.add_method("add_assets", |_lua, this, assets : Variadic<Asset>| {
            let Some(object) = this.0.upgrade() else { return Ok(()) };
            lua_object_add_assets( &object, assets)
        });
        
        methods.add_method("add_actions", |_lua, this, actions : Variadic<ActionDef>| {
            Ok(())
        });
        
        enable_meta_methods_for_ref::<Object, M>(methods);
    }
}

//==============================================================================================
//        LuaObjectDerive
//==============================================================================================

struct LuaObjectDeriveRef(Weak<RwLock<Object>>, String);

impl UserData for LuaObjectDeriveRef {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method("get", |lua, this, ()| {
            let Some(reference) = this.0.upgrade() else { return Ok(Value::Nil) };
            lua_provider_get(lua, &reference, &this.1)
        });
    }
}

//==============================================================================================
//        LuaObjectStat
//==============================================================================================

struct LuaObjectStatRef(Weak<RwLock<Object>>, String);

impl UserData for LuaObjectStatRef {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |lua, this, ()| {
            let Some(reference) = this.0.upgrade() else { return Ok(Value::Nil) };
            lua_provider_get(lua, &reference, &this.1)
        });
        
        methods.add_method("set", |lua, this, value : StatValue| {
            let Some(reference) = this.0.upgrade() else { return Ok(()) };
            lua_stat_set(lua, &reference, &this.1, value)
        });
    }
}

//==============================================================================================
//        Enable Objects
//==============================================================================================

pub fn enable_objects(lua : &Lua) -> FreResult<()> {  
    lua.globals().set("object", lua.create_function(|_lua, (id, statblock) : (String, StatBlock)| {
         Ok(Object::new(&id, statblock))
    })?)?;
    
    Ok(())
}