use std::collections::HashMap;

use mlua::{Function, Lua};

use crate::stat::{derive::StatDerive, field::{StatBlockField, StatProviderRef}, source::StatSource, statblock::StatBlock, value::StatValue};

pub mod source;
pub mod derive;
pub mod field;
pub mod value;
pub mod statblock;
pub mod query;

//==============================================================================================
//        Enable Stats
//==============================================================================================

pub fn enable_stats(lua : &Lua) -> mlua::Result<()> {
    lua.globals().set("stat", lua.create_function(|_lua : &Lua, value : StatValue| {
        let stat = StatSource::new(value);
        return Ok(stat);
    })?)?;
    
    lua.globals().set("derive", lua.create_function(|lua : &Lua, (values, callback) : (Vec<StatProviderRef>, Function)| {
        let derive = StatDerive::new_lua_ref(lua, values, callback)?;
        return Ok(derive);
    })?)?;
    
    lua.globals().set("statblock", lua.create_function(|_lua : &Lua, value : HashMap<String, StatBlockField>| {
        let statblock : StatBlock = value.into();
        Ok(statblock)
    })?)?;
    
    Ok(())
}