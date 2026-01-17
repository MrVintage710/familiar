use std::{collections::HashMap, path::{Path, PathBuf}};

use mlua::{AnyUserData, ExternalResult, FromLua, Lua, UserDataRef, Value};
use pak_db::{builder::PakBuilder, Pak};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{constructor::Constructor, error::{BuildError, FreError, FreException, FreResult}, feature::Feature, lua::run_file_with_lua, object::Object};

const BUILD_SCRIPT_NAME: &str = "build.lua";

pub struct Rulebook {
    name : String,
    pub file : Pak
}

impl Rulebook {
    /// This function runs the build file at the given folder path.
    pub fn build(path : impl AsRef<Path>) -> Result<Self, FreError> {
        //Make path buf
        let path = PathBuf::from(path.as_ref());
        
        //Check directory
        let build_file_path = Self::check_dir(&path)?;
        
        //Setup the lua environment
        // let build_src = LuaSource::new(&build_file_path)?;
        let lua = Lua::new();
        
        //Set default globals to set
        lua.globals().set("name", "rulebook")?;
        lua.globals().set("version", "1.0.0")?;
        lua.globals().set("game_system", "unknown")?;
        lua.globals().set("deps", Vec::<String>::new())?;
        
        //Add the pak builder so that the file will be built
        lua.set_app_data(PakBuilder::new());
        lua.set_app_data(RulebookMeta::default());
        
        lua.globals().set("register", lua.create_function(|lua : &Lua, user_data : AnyUserData| {
            if let Some(mut pak_builder) = lua.app_data_mut::<PakBuilder>() {
                if let Ok(object) = user_data.borrow::<Object>() {
                    pak_builder.pak(&*object).into_lua_err()?;
                    return Ok(());
                }
                
                if let Ok(feature) = user_data.borrow::<Feature>() {
                    pak_builder.pak(&*feature).into_lua_err()?;
                    return Ok(());
                }
                
                if let Ok(constructor) = user_data.borrow::<Constructor>() {
                    let pointer = pak_builder.pak(&*constructor).into_lua_err()?;
                    println!("Registering constructor to {pointer:?}");
                    return Ok(());
                }
            }
            return Ok(());
        })?)?;
        
        // lua.globals().set("register_feature", lua.create_function(|lua : &Lua, feature : FeatureDef| {
        //     if let Some(mut pak_builder) = lua.app_data_mut::<PakBuilder>() {
        //         pak_builder.pak(feature).expect("Error while building feature.");
        //     }
        //     return Ok(Value::Nil);
        // })?)?;
        
        //Run the build script
        run_file_with_lua::<Value>(&lua, build_file_path)?;
        
        //Meta Variables
        let name : String = lua.globals().get("name").unwrap();
        let version : String = lua.globals().get("version").unwrap();
        let game_system : String = lua.globals().get("game_system").unwrap();
        let deps : Vec<RulebookDependency> = lua.globals().get("deps").unwrap();
        
        //Remove the pak builder after use
        let mut pak_builder = lua.remove_app_data::<PakBuilder>().unwrap();
        
        let pak_path = path.clone().join(format!("{}-{version}.rulebook", name.to_lowercase().replace(" ", "-")));
        pak_builder.set_version(version);
        pak_builder.set_name(&name);
        pak_builder.set_extra(&RulebookMeta { game_system, deps })?;
        let pak = pak_builder.build_file(pak_path)?;
        return Ok(Rulebook { name, file: pak });
    }
    
    pub fn check_dir(path : &PathBuf) -> FreResult<PathBuf> {
        //Check and make sure path is a folder
        if !path.is_dir() {return Err(BuildError::BuildPathMustBeFolder(path.clone()).into())}
        
        //Make sure that the file has a build.lua
        let build_file_path = path.clone().join(BUILD_SCRIPT_NAME);
        if !build_file_path.exists() { return Err(BuildError::BuildFileNotFound(build_file_path).into()) }
        
        Ok(build_file_path)
    }
}

//==============================================================================================
//        BuildMeta
//==============================================================================================

#[derive(Serialize, Deserialize, Default)]
pub struct RulebookMeta {
    pub game_system : String,
    pub deps : Vec<RulebookDependency>
}

//==============================================================================================
//        RulebookDependency
//==============================================================================================

#[derive(Serialize, Deserialize)]
pub struct RulebookDependency {
    rulebook: String,
    version: String,
    optional: bool
}

impl TryFrom<String> for RulebookDependency {
    
    type Error = FreError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        let optional = value.ends_with("?");
        if optional { value.pop(); }
        let mut split = value.split("@");
        let Some(rulebook) = split.next() else {return Err(FreError::InvalidDepencyString("Wrong format. Must follow this format: `(rulebook)@(version)`".to_string()))};
        let Some(version) = split.next() else {return Err(FreError::InvalidDepencyString(format!("Missing version. This should be the format: `{rulebook}@(version)`")))};
        Ok(RulebookDependency { rulebook : rulebook.to_string(), version : version.to_string(), optional })
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