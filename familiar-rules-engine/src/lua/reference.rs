use std::sync::{Arc, RwLock};

use mlua::{Lua, UserData};

//==============================================================================================
//        LuaRef
//==============================================================================================

pub(crate) trait LuaRef : Default {
    type RefType : UserData;
    
    fn as_lua_ref(&mut self, lua : &Lua, callback : impl FnOnce(&Lua, Self::RefType) -> mlua::Result<()>) -> mlua::Result<()> {
        let this = Arc::new(RwLock::new(std::mem::take(self)));
        let reference = Self::make_ref(&this);
        
        callback(lua, reference)?;
        
        let this = Arc::into_inner(this).unwrap().into_inner().unwrap();
        *self = this;
        Ok(())
    }
    
    fn from_ref(reference : &Self::RefType) -> Option<Arc<RwLock<Self>>>;
    
    fn make_ref(this : &Arc<RwLock<Self>>) -> Self::RefType;
}