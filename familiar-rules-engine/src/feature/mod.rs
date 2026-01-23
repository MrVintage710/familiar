use mlua::{ExternalResult, FromLua, Function, Lua, UserData, Value};
use pak_db::index::{Indices, PakSearchable};
use serde::{Deserialize, Serialize};

use crate::{common::{choice::Input, meta::{HasItemMeta, ItemMeta, enable_meta_methods}}, error::FreResult, lua::{reference::LuaRef, run_function_with_lua}};

//==============================================================================================
//        
//==============================================================================================

//==============================================================================================
//        Feature
//==============================================================================================

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct Feature {
    setup : Option<Vec<u8>>,
    repeats : (Option<Vec<u8>>, Option<u32>),
    apply : Option<Vec<u8>>,
    meta : ItemMeta
    // input : Option<Vec<u8>>,
    // desc : Option<Vec<u8>>,
    // apply : Option<Vec<u8>>,
    // prereq : Option<Vec<u8>>,
    // local_functions : Vec<Vec<u8>>,
}

impl Feature {
    pub fn new(name: String) -> Self {
        Self { meta : ItemMeta::new(&name, "Feature"),  ..Default::default()}
    }
    
    // pub fn choices(&self) -> Iter<'_, String, Inputs> {
    //     self.inputs.iter()
    // }
    
    // pub fn is_valid(&self) -> bool {
    //     self.inputs.iter().all(|entry| entry.1.is_complete())
    // }
    
    // fn lua_input(&mut self, lua : &Lua, name : String, callback : Function) -> mlua::Result<()> {
    //     let inputs = Inputs::new(name.clone());
    //     let reference = inputs.load(lua);
    //     callback.call::<()>(reference.clone())?;
    //     let input = Inputs::unload(lua, &reference).unwrap();
    //     self.inputs.insert(name, input);
    //     Ok(())
    // }
    
    pub fn evoke_setup(&self, lua : &Lua) -> FreResult<Input> {
        let mut input = Input::default();
        if let Some(callback) = &self.setup {
            input.as_lua_ref(lua, |_, input| {
                run_function_with_lua(lua, callback, input).into_lua_err()
            })?;
            Ok(input)
        } else { Ok(input) }
    }
    
    fn lua_set_repeats_function(&mut self, callback : Function) {
        self.repeats.0 = Some(callback.dump(false));
    }
    
    fn lua_set_apply(&mut self, callback : Function) {
        self.apply = Some(callback.dump(false));
    }
    
    fn lua_set_setup(&mut self, callback : Function) {
        self.setup = Some(callback.dump(false));
    }
    
    pub fn get_apply_function(&self, lua : &Lua) -> Option<Function> {
        let Some(callback) = &self.apply else { return None };
        let Ok(function) = lua.load(callback).into_function() else {return None};
        return Some(function);
    }
    
    // pub fn evoke_apply(&self, lua : &Lua, object : &mut StatBlockData) -> VreResult<()> {
    //     let Some(on_add) = &self.apply else {
    //         return Ok(());
    //     };
    //     let function = lua.load(on_add).into_function()?;
    //     object.scope(lua, |_lua, value| {
    //         function.call::<()>(value)?;
    //         Ok(())
    //     })?;
    //     Ok(())
    // }
    
    // pub fn evoke_prereq(&self, lua : &Lua, object : &mut StatBlockData) -> VreResult<bool> {
    //     let Some(prereqs) = &self.prereq else {
    //         return Ok(true);
    //     };
    //     let function = lua.load(prereqs).into_function()?;
    //     let passes = object.scope(lua, |_lua, value| {
    //         function.call::<bool>(value)
    //     })?;
    //     Ok(passes)
    // }

    // pub fn evoke_setup(&self, lua : &Lua) -> VreResult<FeatureOptions> {
    //     let Some(setup) = &self.input else {
    //         return Ok(FeatureOptions::default());
    //     };
    //     let function = lua.load(setup).into_function()?;
    //     let (builder_ref, lua_options_builder) = FeatureOptions::default().into_lua_builder();
    //     function.call::<()>(lua_options_builder)?;
    //     let options = FeatureOptions::from_lua_builder(builder_ref);
    //     Ok(options)
    // }
}

impl UserData for Feature {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // methods.add_method_mut("input", |lua : &Lua, this : &mut Self, (name, callback) : (String, Function)| {
        //     this.lua_input(lua, name, callback)
        // });
        
        methods.add_method_mut("repeats", |_lua : &Lua, this : &mut Self, callback : Function| {
            this.lua_set_repeats_function(callback);
            Ok(())
        });
        
        methods.add_method_mut("apply", |_lua : &Lua, this : &mut Self, callback : Function| {
            this.lua_set_apply(callback);
            Ok(())
        });
        
        methods.add_method_mut("setup", |_lua : &Lua, this : &mut Self, callback : Function| {
            this.lua_set_setup(callback);
            Ok(())
        });
        
        enable_meta_methods(methods);
    }
}

impl HasItemMeta for Feature {
    fn get_meta(&self) -> &ItemMeta {
        &self.meta
    }

    fn get_meta_mut(&mut self) -> &mut ItemMeta {
        &mut self.meta
    }
}

impl FromLua for Feature {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        let Value::UserData(data) = value else {return Err(mlua::Error::FromLuaConversionError { from: value.type_name(), to: "Feature".to_string(), message: None }) };
        data.take()
    }
}

impl PakSearchable for Feature {
    fn get_indices(&self, indices : &mut Indices) {
        self.meta.get_indices(indices)
    }
}

//==============================================================================================
//        Setup function
//==============================================================================================

pub fn enable_features(lua : &Lua) -> FreResult<()> {  
    lua.globals().set("feature", lua.create_function(|lua, name : String| {
         let feature = Feature::new(name.clone());
         Ok(feature)
    })?)?;
    
    Ok(())
}