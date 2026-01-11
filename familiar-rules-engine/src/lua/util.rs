use mlua::{FromLua, IntoLua, Lua, Value};

//==============================================================================================
//        LuaTableIndex
//==============================================================================================

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LuaTableIndex {
    String(String),
    Integer(i64)
}

impl LuaTableIndex {
    pub fn is_integer(&self) -> bool {
        match self {
            LuaTableIndex::String(_) => false,
            LuaTableIndex::Integer(_) => true,
        }
    }
}

impl FromLua for LuaTableIndex {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Integer(i) => Ok(Self::Integer(i)),
            Value::String(s) => Ok(Self::String(s.to_string_lossy())),
            _ => Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "LuaTableIndex".to_string(), message: None })
        }
    }
}

impl IntoLua for LuaTableIndex {
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        match self {
            LuaTableIndex::String(s) => s.into_lua(lua),
            LuaTableIndex::Integer(i) => i.into_lua(lua),
        }
    }
}

//==============================================================================================
//        Util Types
//==============================================================================================

#[derive(PartialEq, PartialOrd)]
enum LuaComparable {
    Number(f64),
    Integer(i64),
    Bool(bool),
    String(String),
}

impl FromLua for LuaComparable {
    fn from_lua(value: Value, _lua: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Boolean(b) => Ok(Self::Bool(b)),
            Value::Integer(i) => Ok(Self::Integer(i)),
            Value::Number(f) => Ok(Self::Number(f)),
            Value::String(s) => Ok(Self::String(s.to_string_lossy())),
            _ => Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "LuaComparable".to_string(), message: None })
        }
    }
}