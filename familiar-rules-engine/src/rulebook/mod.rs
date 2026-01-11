use std::{collections::HashMap, path::{Path, PathBuf}};

use mlua::{AnyUserData, Lua, UserDataRef, Value};
use pak_db::{builder::PakBuilder, Pak};
use uuid::Uuid;
use crate::{error::{BuildError, VreError}, feature::Feature, lua::{run_file_with_lua}, object::Object};

const BUILD_SCRIPT_NAME: &str = "build.lua";

pub struct Rulebook {
    name : String,
    file : Pak
}

impl Rulebook {
    /// This function runs the build file at the given folder path.
    pub fn build(path : impl AsRef<Path>) -> Result<Self, VreError> {
        //Make path buf
        let path = PathBuf::from(path.as_ref());
        
        //Check and make sure path is a folder
        if !path.is_dir() {return Err(BuildError::BuildPathMustBeFolder(path.clone()).into())}
        
        //Make sure that the file has a build.lua
        let build_file_path = path.clone().join(BUILD_SCRIPT_NAME);
        if !build_file_path.exists() { return Err(BuildError::BuildFileNotFound(build_file_path).into()) }
        
        //Setup the lua environment
        // let build_src = LuaSource::new(&build_file_path)?;
        let lua = Lua::new();
        
        //Set default globals to set
        lua.globals().set("name", "rulebook")?;
        
        //Add the pak builder so that the file will be built
        lua.set_app_data(PakBuilder::new());
        lua.set_app_data(RulebookMeta::default());
        
        lua.globals().set("register", lua.create_function(|lua : &Lua, user_data : AnyUserData| {
            if let Some(mut pak_builder) = lua.app_data_mut::<PakBuilder>() {
                if let Ok(object) = user_data.borrow::<Object>() {
                    
                    pak_builder.pak(&*object).map_err(|e| mlua::Error::MemoryError(e.to_string()))?;
                    return Ok(());
                }
                
                if let Ok(feature) = user_data.borrow::<Feature>() {
                    pak_builder.pak(&*feature).map_err(|e| mlua::Error::MemoryError(e.to_string()))?;
                    return Ok(());
                }
            }
            return Ok(());
        })?)?;
        
        lua.globals().set("rulebook_name", lua.create_function(|lua : &Lua, name : String| {
            if let Some(mut pak_builder) = lua.app_data_mut::<PakBuilder>() {
                pak_builder.set_name(&name);
            }
            Ok(())
        })?)?;
        
        // lua.globals().set("register_feature", lua.create_function(|lua : &Lua, feature : FeatureDef| {
        //     if let Some(mut pak_builder) = lua.app_data_mut::<PakBuilder>() {
        //         pak_builder.pak(feature).expect("Error while building feature.");
        //     }
        //     return Ok(Value::Nil);
        // })?)?;
        
        //Run the build script
        run_file_with_lua::<Value>(&lua, build_file_path)?;
        
        //Remove the pak builder after use
        let pak_builder = lua.remove_app_data::<PakBuilder>().unwrap();
        let name : String = lua.globals().get("name").unwrap();
        let pak_path = path.clone().join(format!("{name}.pak"));
        let pak = pak_builder.build_file(pak_path)?;
        return Ok(Rulebook { name, file: pak });
    }
}

//==============================================================================================
//        BuildMeta
//==============================================================================================

#[derive(Default)]
pub struct RulebookMeta {
    features : HashMap<Uuid, Feature>,
    objects : HashMap<Uuid, Object>
}