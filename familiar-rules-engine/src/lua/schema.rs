

use std::collections::HashMap;

use mlua::Value;
use serde::{Deserialize, Serialize};

use crate::error::{SchemaError, SchemaResult, VreError, VreResult};

//==============================================================================================
//        ValueSchema
//==============================================================================================

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueSchema {
    #[default]
    Nil,
    Boolean,
    LightUserData,
    Integer,
    Number,
    String,
    Table(HashMap<String, ValueSchema>),
    Function,
    Thread,
    UserData,
    Error,
    Other,
}

impl ValueSchema {
    pub fn type_name(&self) -> &'static str {
        match self {
            ValueSchema::Nil => "nil",
            ValueSchema::Boolean => "boolean",
            ValueSchema::LightUserData => "lightuserdata",
            ValueSchema::Integer => "integer",
            ValueSchema::Number => "number",
            ValueSchema::String => "string",
            ValueSchema::Table(_) => "table",
            ValueSchema::Function => "function",
            ValueSchema::Thread => "thread",
            ValueSchema::UserData => "userdata",
            ValueSchema::Error => "error",
            ValueSchema::Other => "other",
        }
    }
    
    pub fn derive(value : &Value) -> VreResult<Self> {
        match value {
            Value::Nil => Ok(ValueSchema::Nil),
            Value::Boolean(_) => Ok(ValueSchema::Boolean),
            Value::LightUserData(_) => Ok(ValueSchema::LightUserData),
            Value::Integer(_) => Ok(ValueSchema::Integer),
            Value::Number(_) => Ok(ValueSchema::Number),
            Value::String(_) => Ok(ValueSchema::String),
            Value::Table(table) => {
                let mut map = HashMap::new();
                for pair in table.pairs::<String, Value>() {
                    let (key, value) = pair.map_err(|e| VreError::LuaError(e))?;
                    map.insert(key, ValueSchema::derive(&value)?);
                }
                Ok(ValueSchema::Table(map))
            },
            Value::Function(_) => Ok(ValueSchema::Function),
            Value::Thread(_) => Ok(ValueSchema::Thread),
            Value::UserData(_) => Ok(ValueSchema::UserData),
            Value::Error(_) => Ok(ValueSchema::Error),
            Value::Other(_) => Ok(ValueSchema::Other),
        }
    }
    
    pub fn check(&self, value : &Value) -> bool {
        match self.check_err(value) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    pub fn check_err(&self, value : &Value) -> SchemaResult<()> {
        self.check_recursive(value, Vec::new())
    }
    
    fn check_recursive(&self, value : &Value, current_path: Vec<String>) -> SchemaResult<()>  {
        let path = current_path.join(".");
        match (self, value) {
            (ValueSchema::Table(map), Value::Table(table)) => {
                for (key, expected) in map {
                    let Ok(actual) = table.get::<Value>(key.as_str()) else {
                        return Err(SchemaError::MissingKey { key: key.clone(), path })
                    };
                    let mut new_path = current_path.clone();
                    new_path.push(key.clone());
                    expected.check_recursive(&actual, new_path)?;
                }
                Ok(())
            },
            
            (ValueSchema::Nil, Value::Nil) |
            (ValueSchema::Boolean, Value::Boolean(_)) |
            (ValueSchema::LightUserData, Value::LightUserData(_)) |
            (ValueSchema::Integer, Value::Integer(_)) |
            (ValueSchema::Number, Value::Number(_)) |
            (ValueSchema::String, Value::String(_)) |
            (ValueSchema::Function, Value::Function(_)) |
            (ValueSchema::Thread, Value::Thread(_)) |
            (ValueSchema::UserData, Value::UserData(_)) |
            (ValueSchema::Error, Value::Error(_)) |
            (ValueSchema::Other, Value::Other(_)) => Ok(()),
            
            _ => Err(SchemaError::TypeMismatch { expected: self.type_name().to_string(), actual: value.type_name().to_string(), path })
        }
    }
}
