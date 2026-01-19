pub mod util;
pub mod iter;
// pub mod source;
pub mod schema;
pub mod reference;

use std::{collections::HashMap, path::{Path, PathBuf}};
use mlua::{AppDataRefMut, AsChunk, ExternalResult, FromLuaMulti, IntoLuaMulti, Lua, MultiValue};
use serde::ser::Error;
use crate::{action::enable_actions, asset::enable_assets, constructor::enable_constructor, error::FreResult, feature::enable_features, lua::iter::enable_iter, object::enable_objects, stat::{enable_stats, query::enable_query}};

//==============================================================================================
//        Functions for running lua
//==============================================================================================

pub fn run_file<R : FromLuaMulti>(path : impl AsRef<Path>) -> FreResult<R> {
    let lua = Lua::new();
    run_file_with_lua(&lua, path)
}

pub fn run_file_with_lua<R : FromLuaMulti>(lua : &Lua, path : impl AsRef<Path>) -> FreResult<R> {
    enable_apis(lua, Some(path.as_ref()))?;
    
    let Some(meta) = lua.app_data_mut::<LuaSourceMeta>() else { 
        return Err(mlua::Error::custom("Missing metadata required to run the current file.").into()) 
    };
    
    execute_current_file(lua, meta)
}

pub fn run_function<R : FromLuaMulti>(function : impl AsChunk, args : impl IntoLuaMulti) -> FreResult<R> {
    let lua = Lua::new();
    run_function_with_lua(&lua, function, args)
}

pub fn run_function_with_lua<R : FromLuaMulti>(lua : &Lua, function : impl AsChunk, args : impl IntoLuaMulti) -> FreResult<R> {
    enable_apis::<String>(lua, None)?;
    let function = lua.load(function).into_function()?;
    Ok(function.call(args)?)
}

fn execute_current_file<R : FromLuaMulti>(lua : &Lua, meta : AppDataRefMut<LuaSourceMeta>) -> FreResult<R> {
    if let Some(result) = meta.sources.get(&meta.current_file) {
        Ok(R::from_lua_multi(result.clone(), lua)?)
    } else {
        let path = meta.current_file.clone();
        if !path.exists() || path.is_dir() { 
            return Err(mlua::Error::custom("The required file must a valid lua file.").into())
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let source = std::fs::read_to_string(&path)?;
        drop(meta);
        let result = lua.load(source).set_name(name).eval::<MultiValue>()?;
        if let Some(mut meta) = lua.app_data_mut::<LuaSourceMeta>() {
            meta.sources.insert(path, result.clone());
        }
        Ok(R::from_lua_multi(result, lua)?)
    }
}

pub(crate) fn enable_apis<P : AsRef<Path>>(lua : &Lua, starting_file : Option<P>) -> FreResult<()> {
    enable_features(lua)?;
    enable_iter(lua)?;
    enable_objects(lua)?;
    if let Some(path) = starting_file {
        enable_require(lua, path)?;
    }
    enable_actions(lua)?;
    enable_stats(lua)?;
    enable_query(lua)?;
    enable_constructor(lua)?;
    enable_assets(lua)?;
    Ok(())
}

//==============================================================================================
//        Lua Source Meta
//==============================================================================================

#[derive(Debug)]
pub struct LuaSourceMeta {
    pub current_file : PathBuf,
    pub sources : HashMap<PathBuf, MultiValue>,
}

pub fn lua_get_current_file_name(lua : &Lua) -> mlua::Result<String> {
    let lua_meta = lua.app_data_ref::<LuaSourceMeta>().ok_or(mlua::Error::custom("Missing Lua meta."))?;
    let file = lua_meta.current_file.file_name().unwrap().to_str().unwrap().to_string();
    Ok(file)
}

//==============================================================================================
//        Require
//==============================================================================================


fn enable_require(lua : &Lua, starting_file : impl AsRef<Path>) -> FreResult<()> {
    lua.set_app_data(LuaSourceMeta {
        current_file: PathBuf::from(starting_file.as_ref()),
        sources: HashMap::default(),
    });
    
    lua.globals().set("require", lua.create_function(|lua : &Lua, rel_path : String| {
        let Some(mut meta) = lua.app_data_mut::<LuaSourceMeta>() else { 
            return Err(mlua::Error::custom("Cannot call require from a non global context."))
        };
        let new_file_path = meta.current_file.parent().unwrap().to_path_buf().join(format!("{rel_path}.lua"));
        let current_path = meta.current_file.clone();
        meta.current_file = new_file_path;
        let result = execute_current_file::<MultiValue>(lua, meta).into_lua_err()?;
        if let Some(mut meta) = lua.app_data_mut::<LuaSourceMeta>() {
            meta.current_file = current_path;
        }
        Ok(result)
    })?)?;
    Ok(())
}