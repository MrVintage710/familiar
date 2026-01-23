use mlua::{FromLua, Function, Lua, UserData, Value};
use ordermap::OrderMap;
use pak_db::index::{Indices, PakSearchable};
use serde::{Deserialize, Serialize};

use crate::{common::{choice::Input, meta::{HasItemMeta, ItemMeta, enable_meta_methods}}, error::FreResult, lua::reference::LuaRef, object::Object, stat::statblock::StatBlock};

//==============================================================================================
//        
//==============================================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constructor {
    meta : ItemMeta,
    finalize : Option<Vec<u8>>,
    steps : OrderMap<String, Input>
}

impl Constructor {
    pub fn new(name : &str) -> Self {
        Self {
            meta : ItemMeta::new(name, "Constructor"),
            finalize: None,
            steps : OrderMap::default()
        }
    }
    
    pub fn steps(&self) -> ordermap::map::Iter<'_, String, Input>  {
        self.steps.iter()
    }
    
    pub fn steps_mut(&mut self) -> ordermap::map::IterMut<'_, String, Input> {
        self.steps.iter_mut()
    }
}

impl UserData for Constructor {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("step", |lua, this, (name, callback) : (String, Function)| {
            let mut step = Input::new();
            step.as_lua_ref(lua, |_, value| {
                callback.call(value)
            })?;
            this.steps.insert(name, step);
            Ok(())
        });
        
        methods.add_method_mut("finalize", |_lua, this, callback : Function| {
            this.finalize = Some(callback.dump(false));
            Ok(())
        });
        
        enable_meta_methods(methods);
    }
}

impl HasItemMeta for Constructor {
    fn get_meta(&self) -> &ItemMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut ItemMeta {
        &mut self.meta
    }
}

impl FromLua for Constructor {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::UserData(data) = value else {return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Constructor".to_string(), message: None }) };
        data.take()
    }
}

impl PakSearchable for Constructor {
    fn get_indices(&self, indices : &mut Indices) {
        self.meta.get_indices(indices);
    }
}

//==============================================================================================
//        Constructor Lua
//==============================================================================================

pub fn enable_constructor(lua : &Lua) -> FreResult<()> {  
    lua.globals().set("constructor", lua.create_function(|_, name : String| {
        let constructor = Constructor::new(&name);
        Ok(constructor)
    })?)?;
    
    Ok(())
}