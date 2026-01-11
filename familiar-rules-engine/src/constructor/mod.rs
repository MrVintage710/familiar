use std::{collections::VecDeque, sync::{Arc, RwLock}};

use mlua::{Function, Lua, UserData};

use crate::{common::{enable_meta_methods, HasItemMeta, ItemMeta}, error::VreResult, object::Object, stat::statblock::StatBlock};

//==============================================================================================
//        
//==============================================================================================

pub struct Constructor {
    meta : ItemMeta,
    steps : VecDeque<ConstructorStep>
}

impl Constructor {
    pub fn new(name : &str) -> Self {
        Self {
            meta : ItemMeta::new(name, "Constructor"),
            steps : VecDeque::default()
        }
    }
}

impl UserData for Constructor {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("step", |lua, this, (name, callback) : (String, Function)| {
            let step = ConstructorStep::new(name);
            
            
            
            Ok(())
        });
        enable_meta_methods(methods);
    }
}

impl HasItemMeta for Constructor {
    fn get_meta(&self) -> &ItemMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut ItemMeta {
        &mut self.meta
    }
}

//==============================================================================================
//        Constructor Step
//==============================================================================================

pub struct ConstructorStep {
    id : String,
    inputs : VecDeque<ConstructorInput>
}

impl ConstructorStep {
    pub fn new(id : String) -> Self {
        Self {
            id,
            inputs: VecDeque::default(),
        }
    }
}

//==============================================================================================
//        ConstructorPageInput
//==============================================================================================

pub struct ConstructorInput {
    id : String,
    renderer : Option<String>,
    kind : ConstructorPageInputKind
}

pub enum ConstructorPageInputKind {
    String(String),
    Number(i64),
    Group(VecDeque<ConstructorInput>)
}

//==============================================================================================
//        Lua Contructor Step
//==============================================================================================

pub struct LuaConstructorStepBuilder {
    step : Arc<RwLock<ConstructorStep>>
}

impl UserData for LuaConstructorStepBuilder {
    
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        
    }
}

//==============================================================================================
//        Enable Constructor
//==============================================================================================

// pub fn enable_objects(lua : &Lua) -> VreResult<()> {  
//     lua.globals().set("constructor", lua.create_function(|_lua, id : String| {
//          Ok(Constructor::new(&id))
//     })?)?;
    
//     Ok(())
// }