use mlua::{FromLua, Lua, UserData, Value};
use serde::{ser::Error, Deserialize, Serialize};

use crate::{error::VreResult};

//==============================================================================================
//        Query
//==============================================================================================

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Query(String);

impl Query {
    pub fn get_query(&self) -> &str {
        &self.0
    }
}

impl UserData for Query {
    
}

impl FromLua for Query {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        let error = mlua::Error::custom(format!("Can not infer query from `{}`", value.type_name()));
        if let Some(string) = value.as_string() {
            return Ok(Query(string.to_string_lossy()))
        }
        
        if let Some(userdata) = value.as_userdata() {
            if let Ok(query) = userdata.borrow::<Query>() {
                return Ok(query.clone())
            }
        }
        
        Err(error)
    }
}

//==============================================================================================
//        Enable Query
//==============================================================================================

pub fn enable_query(lua : &Lua) -> VreResult<()> {
    lua.globals().set("query", lua.create_function(|_lua, query : String| {
         Ok(Query(query))
    })?)?;
    
    Ok(())
}