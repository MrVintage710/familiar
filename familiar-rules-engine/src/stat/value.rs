use std::{collections::HashMap, fmt::Debug};

use mlua::{FromLua, IntoLua, Value};
use pak_db::value::PakValue;
use serde::{Deserialize, Serialize};

use crate::stat::query::Query;

//==============================================================================================
//        StatValue
//==============================================================================================


#[derive(Serialize, Deserialize, Default, PartialEq, Clone)]
pub enum StatValue {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Record(HashMap<String, StatValue>),
    Array(Vec<StatValue>),
    LuaFunction(Vec<u8>),
    Query(Query),
    #[serde(skip)]
    LuaValue(Value),
    #[default]
    None
}

impl StatValue {
    pub fn is_none(&self) -> bool {
        matches!(self, StatValue::None)
    }
    
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, StatValue::String(_))
    }
    
    pub fn as_string(&self) -> Option<&String> {
        match self {
            StatValue::String(value) => Some(value),
            _ => None
        }
    }
}

impl Debug for StatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => arg0.fmt(f),
            Self::Float(arg0) => arg0.fmt(f),
            Self::Int(arg0) => arg0.fmt(f),
            Self::Bool(arg0) => arg0.fmt(f),
            Self::Record(arg0) => arg0.fmt(f),
            Self::Array(arg0) => arg0.fmt(f),
            Self::LuaFunction(_) => f.write_str("LuaFunction"),
            Self::LuaValue(arg0) => arg0.fmt(f),
            Self::Query(argo) => f.debug_tuple("Query").field(&argo.get_query()).finish(),
            Self::None => write!(f, "None"),
        }
    }
}

impl From<String> for StatValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&'static str> for StatValue {
    fn from(value: &'static str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<f64> for StatValue {
    fn from(value: f64) -> Self {
        StatValue::Float(value)
    }
}

impl From<i64> for StatValue {
    fn from(value: i64) -> Self {
        StatValue::Int(value)
    }
}

impl From<bool> for StatValue {
    fn from(value: bool) -> Self {
        StatValue::Bool(value)
    }
}

impl <T : Into<StatValue>> From<Vec<T>> for StatValue {
    fn from(value: Vec<T>) -> Self {
        Self::Array(value.into_iter().map(|value| value.into()).collect())
    }
}

impl Into<PakValue> for StatValue {
    fn into(self) -> PakValue {
        match self {
            StatValue::String(string) => PakValue::String(string),
            StatValue::Float(float) => PakValue::Float(float.to_bits()),
            StatValue::Int(int) => PakValue::Int(int),
            StatValue::Bool(bool) => PakValue::Boolean(bool),
            StatValue::Array(array) => PakValue::Array(array.into_iter().filter_map(|value| {
                let pak_value : PakValue = value.into();
                if pak_value.is_void() { None }
                else { Some(pak_value) }
            }).collect()),
            _ => PakValue::Void,
        }
    }
} 

impl FromLua for StatValue {
    fn from_lua(value: Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            Value::Nil => Ok(StatValue::None),
            Value::String(value) => Ok(StatValue::String(value.to_string_lossy())),
            Value::Boolean(value) => Ok(StatValue::Bool(value)),
            Value::Number(value) => Ok(StatValue::Float(value)),
            Value::Integer(value) => Ok(StatValue::Int(value)),
            Value::Function(value) => Ok(StatValue::LuaFunction(value.dump(false))),
            Value::Table(_) => {
                if let Ok(map) = HashMap::<String, StatValue>::from_lua(value.clone(), lua) { Ok(StatValue::Record(map)) }
                else if let Ok(array) = Vec::<StatValue>::from_lua(value.clone(), lua) { Ok(StatValue::Array(array)) }
                else { Ok(StatValue::LuaValue(value)) } 
            },
            _ => Ok(StatValue::LuaValue(value))
        }
    }
}

impl IntoLua for StatValue {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        match self {
            StatValue::String(value) => value.into_lua(lua),
            StatValue::Float(value) => value.into_lua(lua),
            StatValue::Int(value) => value.into_lua(lua),
            StatValue::Bool(value) => value.into_lua(lua),
            StatValue::Record(hash_map) => hash_map.into_lua(lua),
            StatValue::Array(stat_values) => stat_values.into_lua(lua),
            StatValue::LuaFunction(bytes) => lua.load(bytes).into_function()?.into_lua(lua),
            StatValue::LuaValue(value) => Ok(value),
            StatValue::Query(query) => query.into_lua(lua),
            StatValue::None => Ok(Value::Nil),
        }
    }
}

//==============================================================================================
//        Value Type
//==============================================================================================

pub enum ValueType {
    String,
    Float,
    Int,
    Bool,
    Record,
    Array,
    Query
}