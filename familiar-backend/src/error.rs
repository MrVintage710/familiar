use std::path::PathBuf;

use tauri::ipc::InvokeError;
use thiserror::Error;
use uuid::Uuid;

pub type FamiliarResult<T> = Result<T, FamiliarError>;
pub type FamiliarException = Result<(), FamiliarError>;

#[derive(Error, Debug)]
pub enum FamiliarError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    
    #[error("{0}")]
    FreError(#[from] fre::error::FreError),
    
    #[error("{0}")]
    UuidError(#[from] uuid::Error),
    
    #[error("Invalid Rulebook: {0}. Make sure that the path is correct and that the file has the `.rulebook` extention.")]
    InvalidRulebook(PathBuf),
    
    #[error("Object `{uuid}` was not found in the {cache_name} cache.")]
    CacheObjectNotFound {
        cache_name : String,
        uuid : Uuid
    },
    
    #[error("Was unable to find the character constructor associated with ruleset `{0}`. The constructor must have the `Character Creator` tag.")]
    UnableToFindCharacterConstructorForRuleset(String)
}

impl FamiliarError {
    pub fn cache_object_not_found(cache_name : &str, uuid: &Uuid) -> Self {
        Self::CacheObjectNotFound { cache_name: cache_name.to_string(), uuid: uuid.clone() }
    }
    
    pub fn missing_character_constructor(ruleset : &str) -> Self {
        Self::UnableToFindCharacterConstructorForRuleset(ruleset.to_string())
    }
}

impl Into<InvokeError> for FamiliarError {
    fn into(self) -> InvokeError {
        InvokeError::from_error(self)
    }
}