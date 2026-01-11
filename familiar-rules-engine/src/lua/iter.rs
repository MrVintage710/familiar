use std::collections::HashMap;

use mlua::{AnyUserData, Function, Lua, ObjectLike, Table, UserData, Value};

use super::util::LuaTableIndex;

//==============================================================================================
//        LuaIter
//==============================================================================================

pub fn enable_iter(lua : &Lua) -> mlua::Result<()> {
    lua.globals().set("iter", lua.create_function(|_lua, map : HashMap<LuaTableIndex, Value>| {
        let map : Vec<(LuaTableIndex, Value)> = map.into_iter().collect();
        let mut iter = LuaIter { map };
        iter.sort();
        Ok(iter)
    })?)?;
    
    lua.globals().set("table_contains", lua.create_function(table_contains)?)?;
    
    Ok(())
}

pub struct LuaIter {
    map : Vec<(LuaTableIndex, Value)>
}

impl LuaIter {    
    fn lua_map(_lua : &Lua, (this, callback) : (AnyUserData, Function)) -> mlua::Result<AnyUserData> {
        for (_key, value) in this.borrow_mut::<Self>()?.map.iter_mut() {
            let result : Value = callback.call(value.clone())?;
            *value = result
        }
        Ok(this.clone())
    }
    
    fn lua_filter(_lua : &Lua, (this, callback) : (AnyUserData, Function)) -> mlua::Result<AnyUserData> {
        this.borrow_mut::<Self>()?.map.retain(|(_key, value)| callback.call(value.clone()).unwrap_or(false));
        this.borrow_mut::<Self>()?.normalize();
        Ok(this.clone())
    }
    
    fn lua_any(_lua : &Lua, (this, callback) : (AnyUserData, Function)) -> mlua::Result<bool> {
        let result = this.borrow_mut::<Self>()?.map.iter().any(|(_key, value)| callback.call(value.clone()).unwrap_or(false));
        Ok(result)
    }
    
    fn lua_collect(lua : &Lua, this : AnyUserData) -> mlua::Result<Table> {
        let this = this.take::<Self>()?;
        lua.create_table_from(this.map)
    }
    
    fn normalize(&mut self) {
        let mut current_index = 1;
        for (key, _) in self.map.iter_mut() {
            let LuaTableIndex::Integer(index) = key else { continue; };
            *index = current_index;
            current_index += 1;
        }
    }
    
    fn sort(&mut self) {
        self.map.sort_by(|(key_a, _), (key_b, _)| {
            match (key_a, key_b) {
                (LuaTableIndex::String(a), LuaTableIndex::String(b)) => a.cmp(b),
                (LuaTableIndex::String(_), LuaTableIndex::Integer(_)) => std::cmp::Ordering::Greater,
                (LuaTableIndex::Integer(_), LuaTableIndex::String(_)) => std::cmp::Ordering::Less,
                (LuaTableIndex::Integer(a), LuaTableIndex::Integer(b)) => a.cmp(b),
            }
        });
    }
}

impl UserData for LuaIter {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("map", Self::lua_map);
        methods.add_function("filter", Self::lua_filter);
        methods.add_function("any", Self::lua_any);
        methods.add_function("collect", Self::lua_collect);
    }
}

pub fn table_contains(_lua : &Lua, (table, value) : (Table, Value)) -> mlua::Result<bool> {
    for pair in table.pairs::<Value, Value>() {
        if pair?.1 == value { return Ok(true)}
    }
    
    Ok(false)
}

