use mlua::{Lua, Result};

use crate::action::definition::ActionDef;

pub mod definition;

//==============================================================================================
//        Action
//==============================================================================================

pub struct Action {
    resolver : Vec<u8>
}

//==============================================================================================
//        Add Action
//==============================================================================================

pub fn enable_actions(lua : &Lua) -> Result<()> {
    lua.globals().set("action", lua.create_function(|_ : &Lua, name : String| {
        let action_def = ActionDef::new(name.as_str());
        return Ok(action_def);
    })?)?;
    Ok(())
}