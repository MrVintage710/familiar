use mlua::{FromLua, Function, Lua, UserData, UserDataRefMut, Value};
use pak_db::index::{Indices, PakSearchable};
use serde::{Deserialize, Serialize};

use crate::common::{enable_meta_methods, HasItemMeta, ItemMeta};

//===============================================================================================
//          ActionDefinition
//===============================================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionDef {
    meta : ItemMeta,
    resolver: Option<Vec<u8>>
}

impl ActionDef {
    pub fn new(name : &str) -> Self {
        Self {
            meta : ItemMeta::new(name, "Action"),
            resolver: None
        }
    }
}

impl PakSearchable for ActionDef {
    fn get_indices(&self, indices : &mut Indices) {
        self.meta.get_indices(indices)
    }
}

impl HasItemMeta for ActionDef {
    fn get_meta(&self) -> &ItemMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut ItemMeta {
        &mut self.meta
    }
}

impl UserData for ActionDef {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_function_set("resolver", |_, this, value : Function| {
            let mut this : UserDataRefMut<ActionDef> = this.borrow_mut()?;
            this.resolver = Some(value.dump(false));
            return Ok(());
        });
    }
    
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        enable_meta_methods(methods);
    }

    fn register(registry: &mut mlua::UserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
    }
}

impl FromLua for ActionDef {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        let Value::UserData(data) = value else {return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Action".to_string(), message: None }) };
        data.take()
    }
}