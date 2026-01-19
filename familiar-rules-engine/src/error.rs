use std::path::PathBuf;

use base64::DecodeError;
use image::ImageError;
use pak_db::error::PakError;
use thiserror::Error;

pub type FreResult<T> = Result<T, FreError>;
pub type FreException = Result<(), FreError>;
pub type SchemaResult<T> = Result<T, SchemaError>;

//==============================================================================================
//        Errors
//==============================================================================================

#[derive(Error, Debug)]
pub enum FreError {
    #[error("Multiple Errors Found: {0:#?}")]
    MultipleErrors(Vec<FreError>),
    
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
    
    #[error("Image Error: {0}")]
    ImageError(#[from] ImageError),
    
    #[error("Decode Error: {0}")]
    DecodeError(#[from] DecodeError),
    
    #[error("There was a problem parsing a dependancy string: {0}")]
    InvalidDepencyString(String),
    
    #[error("Files with the `{0}` extention are not valid asset files.")]
    FileTypeNotValidAsset(String),
    
    #[error("Missing the mime type for file extention `{0}`.")]
    MissingMimeType(String),
}

impl Into<mlua::Error> for FreError {
    fn into(self) -> mlua::Error {
        match self {
            FreError::LuaError(err) => return err,
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