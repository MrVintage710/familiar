use std::path::PathBuf;

use pak_db::error::PakError;
use thiserror::Error;

pub type VreResult<T> = Result<T, VreError>;
pub type SchemaResult<T> = Result<T, SchemaError>;

//==============================================================================================
//        Errors
//==============================================================================================

#[derive(Error, Debug)]
pub enum VreError {
    #[error("Multiple Errors Found: {0:#?}")]
    MultipleErrors(Vec<VreError>),
    
    #[error("{0}")]
    FileNotFound(#[from] std::io::Error),
    
    #[error("Lua Error: {0}")]
    LuaError(#[from] mlua::Error),
    
    #[error("Lua Conversion Error: {0}")]
    LuaConversionError(#[from] LuaConversionError),
    
    #[error("Schema Check Failed: {0}")]
    SchemaError(#[from] SchemaError),
    
    #[error("Build Error: {0}")]
    BuildError(#[from] BuildError),
    
    #[error("Pak Error: {0}")]
    PakError(#[from] PakError),
}

impl Into<mlua::Error> for VreError {
    fn into(self) -> mlua::Error {
        match self {
            VreError::LuaError(err) => return err,
            _ => {}
        };
        mlua::Error::external(self)
    }
}

//==============================================================================================
//        Schema Errors
//==============================================================================================

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Type Mismatch: Expected {expected}, got {actual} at {path}")]
    TypeMismatch {
        expected: String,
        actual: String,
        path: String,
    },
    
    #[error("Missing Key: {key} at {path}")]
    MissingKey {
        key: String,
        path: String,
    }
}

//==============================================================================================
//        Lua Conversion Errors
//==============================================================================================

#[derive(Error, Debug)]
pub enum LuaConversionError {
    #[error("Cannot convert {from} to {to}.")]
    FromLuaError {
        from: String,
        to: String,
    },
}

//==============================================================================================
//        Lua Conversion Errors
//==============================================================================================

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("The build path `{0}` must be a folder.")]
    BuildPathMustBeFolder(PathBuf),
    
    #[error("The build file at `{0}` was not found.")]
    BuildFileNotFound(PathBuf),
    
    
}