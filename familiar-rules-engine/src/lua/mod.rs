pub mod util;
pub mod iter;
pub mod schema;
pub mod reference;

use std::{collections::HashMap, path::{Path, PathBuf}};
use mlua::{AppDataRefMut, AsChunk, ExternalResult, FromLuaMulti, IntoLuaMulti, Lua, MultiValue};
use serde::ser::Error;
use crate::{action::enable_actions, asset::enable_assets, common::identifier::Identifier, constructor::enable_constructor, error::{FreException, FreResult}, feature::enable_features, lua::iter::enable_iter, object::enable_objects, rulebook::RulebookSettings, stat::{enable_stats, query::enable_query}};

//==============================================================================================
//        Functions for running lua
//==============================================================================================

pub fn run_file<M : LuaRunMode>(lua : &Lua, path : impl AsRef<Path>, run_mode : M) -> FreResult<M::Return> {
    lua_path_scope(lua, path, |lua| {
        let Some(meta) = lua.app_data_mut::<LuaSourceMeta>() else { 
            return Err(mlua::Error::custom("Missing metadata required to run the current file.").into()) 
        };
        
        // execute_current_file(lua, meta)
        run_mode.execute_current_file(lua, meta)
    })
}

fn lua_path_scope<R>(lua : &Lua, path : impl AsRef<Path>, callback : impl FnOnce(&Lua) -> R) -> R {
    let previous = if let Some(mut source_meta) = lua.app_data_mut::<LuaSourceMeta>() {
        let previous = source_meta.current_file.clone();
        source_meta.current_file = PathBuf::from(path.as_ref());
        drop(source_meta);
        previous
    } else {
        lua.set_app_data(LuaSourceMeta {
            current_file: PathBuf::from(path.as_ref()),
            sources: HashMap::default(),
            deps: HashMap::default()
        });
        PathBuf::from(path.as_ref())
    };
    
    let result = callback(lua);
    
    let mut meta = lua.app_data_mut::<LuaSourceMeta>().unwrap();
    meta.current_file = previous;
    result
}

pub fn run_function<R : FromLuaMulti>(function : impl AsChunk, args : impl IntoLuaMulti) -> FreResult<R> {
    let lua = Lua::new();
    run_function_with_lua(&lua, function, args)
}

pub fn run_function_with_lua<R : FromLuaMulti>(lua : &Lua, function : impl AsChunk, args : impl IntoLuaMulti) -> FreResult<R> {
    enable_apis(lua)?;
    let function = lua.load(function).into_function()?;
    Ok(function.call(args)?)
}

pub(crate) fn enable_apis(lua : &Lua) -> FreResult<()> {
    enable_features(lua)?;
    enable_iter(lua)?;
    enable_objects(lua)?;
    enable_require(lua)?;
    enable_dep(lua)?;
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
    pub deps : HashMap<PathBuf, Vec<Identifier>>
}

pub fn lua_get_current_file_name(lua : &Lua) -> mlua::Result<String> {
    let lua_meta = lua.app_data_ref::<LuaSourceMeta>().ok_or(mlua::Error::custom("Missing Lua meta."))?;
    let file = lua_meta.current_file.file_name().unwrap().to_str().unwrap().to_string();
    Ok(file)
}

pub trait LuaRunMode {
    type Return;
    
    fn execute_current_file(self, lua : &Lua, meta : AppDataRefMut<LuaSourceMeta>) -> FreResult<Self::Return>;
}

pub struct LuaRequireRun;

impl LuaRunMode for LuaRequireRun {
    type Return = MultiValue;

    fn execute_current_file(self, lua : &Lua, meta : AppDataRefMut<LuaSourceMeta>) -> FreResult<Self::Return> {
        if let Some(result) = meta.sources.get(&meta.current_file) {
            Ok(result.clone())
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
            Ok(result)
        }
    }
}

#[derive(Default)]
pub struct LuaDepsRun(pub Vec<Identifier>);

impl LuaRunMode for LuaDepsRun {
    type Return = Vec<Identifier>;

    fn execute_current_file(self, lua : &Lua, meta : AppDataRefMut<LuaSourceMeta>) -> FreResult<Self::Return> {
        if let Some(result) = meta.deps.get(&meta.current_file) {
            Ok(result.clone())
        } else {
            let path = meta.current_file.clone();
            if !path.exists() || path.is_dir() { 
                return Err(mlua::Error::custom("The required file must a valid lua file.").into())
            }
            let domestic_uuid = lua.app_data_ref::<RulebookSettings>().ok_or(mlua::Error::custom("Rulebook Settings not in environment."))?.uuid();
            let foreign_uuid = RulebookSettings::from_path(&path).into_lua_err()?.uuid();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let source = std::fs::read_to_string(&path)?;
            lua.set_app_data(self);
            drop(meta);
            lua.load(source).set_name(name).exec()?;
            let mut values = lua.remove_app_data::<LuaDepsRun>().unwrap().0;
            if domestic_uuid != foreign_uuid { values.iter_mut().for_each(|id| id.set_rulebook(foreign_uuid));}
            if let Some(mut meta) = lua.app_data_mut::<LuaSourceMeta>() {
                meta.deps.insert(path, values.clone());
            }
            Ok(values)
        }
    }
}

//==============================================================================================
//        Require
//==============================================================================================


fn enable_require(lua : &Lua) -> FreException {
    lua.globals().set("require", lua.create_function(|lua : &Lua, rel_path : String| {
        let Some(meta) = lua.app_data_ref::<LuaSourceMeta>() else { 
            return Err(mlua::Error::custom("Cannot call require from a non global context."))
        };
        let new_file_path = meta.current_file.parent().unwrap().to_path_buf().join(format!("{rel_path}.lua"));
        drop(meta);
        let result = run_file(lua, new_file_path, LuaRequireRun).into_lua_err()?;
        Ok(result)
    })?)?;
    Ok(())
}

//==============================================================================================
//        enable dep
//==============================================================================================

fn enable_dep(lua : &Lua) -> FreException {
    lua.globals().set("dep", lua.create_function(|lua : &Lua, rel_path : String| {
        let meta = lua.app_data_mut::<LuaSourceMeta>().ok_or(mlua::Error::custom("Cannot call dep from a non global context."))?;
        let path = meta.current_file.parent().unwrap().to_path_buf().join(format!("{rel_path}.lua"));
        drop(meta);
        let result = run_file(lua, path, LuaDepsRun::default()).into_lua_err()?;
        Ok(result)
    })?)?;
    
    Ok(())
}