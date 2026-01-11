pub mod util;
pub mod choice;

use std::collections::HashSet;

use mlua::{Value, Variadic};
use pak_db::index::{Indices, PakSearchable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::lua::reference::LuaRef;

//==============================================================================================
//        HasItemMeta trait
//==============================================================================================

pub trait HasItemMeta {
    fn get_meta(&self) -> &ItemMeta;
    
    fn get_meta_mut(&mut self) -> &mut ItemMeta;
}

//==============================================================================================
//        Item Meta
//==============================================================================================\

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ItemMeta {
    name : String,
    desc : String,
    uuid : Uuid,
    type_name : String,
    tags : HashSet<String>,
}

impl ItemMeta {
    pub fn new(name : &str, type_name : &str) -> Self {
        Self {
            name : name.to_string(),
            type_name : type_name.to_string(),
            desc : String::new(),
            uuid : Uuid::new_v4(),
            tags : HashSet::new(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl PakSearchable for ItemMeta {
    fn get_indices(&self, indices : &mut Indices) {
        indices.add("name", self.name.clone());
        indices.add("desc", self.desc.clone());
        indices.add("type", self.type_name.clone());
        indices.add("uuid", self.uuid.to_string());
        indices.add("tags", self.tags.iter().cloned().collect::<Vec<_>>());
    }
}

pub(crate) fn enable_meta_methods<T : HasItemMeta, F: mlua::UserDataMethods<T>>(methods : &mut F) { 
    methods.add_method_mut("add_tags", |_, this, tags : Variadic<String>| {
        this.get_meta_mut().tags.extend(tags.into_iter());
        Ok(())
    });
    
    methods.add_method_mut("set_tags", |_, this, tags : HashSet<String>| {
        this.get_meta_mut().tags = tags;
        Ok(())
    });
    
    methods.add_method_mut("clear_tags", |_, this, ()| {
        this.get_meta_mut().tags.clear();
        Ok(())
    });
    
    methods.add_method_mut("set_desc", |_, this, desc : String| {
        this.get_meta_mut().desc = desc;
        Ok(())
    });
}

pub(crate) fn enable_meta_methods_for_ref<T : HasItemMeta + LuaRef, F: mlua::UserDataMethods<<T as LuaRef>::RefType>>(methods : &mut F) {
    methods.add_method("add_tags", |_lua, this, tags : Variadic<String>| {
        let Some(value) = T::from_ref(&this) else {return Ok(Value::Nil)};
        let Ok(mut value) = value.write() else {return Ok(Value::Nil)};
        value.get_meta_mut().tags.extend(tags.into_iter());
        Ok(Value::Nil)
    });
    
    methods.add_method("set_tags", |_, this, tags : HashSet<String>| {
        let Some(value) = T::from_ref(&this) else {return Ok(Value::Nil)};
        let Ok(mut value) = value.write() else {return Ok(Value::Nil)};
        value.get_meta_mut().tags = tags;
        Ok(Value::Nil)
    });
    
    methods.add_method_mut("clear_tags", |_, this, ()| {
        let Some(value) = T::from_ref(&this) else {return Ok(Value::Nil)};
        let Ok(mut value) = value.write() else {return Ok(Value::Nil)};
        value.get_meta_mut().tags.clear();
        Ok(Value::Nil)
    });
    
    methods.add_method_mut("set_desc", |_, this, desc : String| {
        let Some(value) = T::from_ref(&this) else {return Ok(Value::Nil)};
        let Ok(mut value) = value.write() else {return Ok(Value::Nil)};
        value.get_meta_mut().desc = desc;
        Ok(Value::Nil)
    });
}

