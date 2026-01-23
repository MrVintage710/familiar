use mlua::{ExternalResult, FromLua, Lua, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::FreError, prelude::Rulebook};


//==============================================================================================
//        RulebookDependency
//==============================================================================================

#[derive(Serialize, Deserialize)]
pub struct RulebookDependency {
    rulebook: String,
    version: String,
    uuid : Uuid,
    optional: bool
}

impl RulebookDependency {
    pub fn new(rulebook: String, version: String) -> Self {
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_X500, format!("{rulebook}|{version}").as_bytes());
        Self { rulebook, version, uuid, optional : false }
    }
}

impl TryFrom<String> for RulebookDependency {
    
    type Error = FreError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let optional = value.ends_with("?");
        if optional { value.pop(); }
        let mut split = value.split("@");
        let Some(rulebook) = split.next() else {return Err(FreError::InvalidDepencyString("Wrong format. Must follow this format: `(rulebook)@(version)`".to_string()))};
        let Some(version) = split.next() else {return Err(FreError::InvalidDepencyString(format!("Missing version. This should be the format: `{rulebook}@(version)`")))};
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_X500, format!("{rulebook}|{version}").as_bytes());
        Ok(RulebookDependency { rulebook : rulebook.to_string(), version : version.to_string(), uuid, optional })
    }
}

impl FromLua for RulebookDependency {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        let type_name = value.type_name();
        if let Ok(str) = String::from_lua( value, lua) {
            Ok(str.try_into().into_lua_err()?)
        } else {
            Err(mlua::Error::FromLuaConversionError { from: type_name, to:std::any::type_name::<Self>().to_string(), message: None })
        }
    }
}

//==============================================================================================
//        Dependency Reference
//==============================================================================================

pub struct DependencyRef {
    rulebook : String,
    uuid : Uuid
}