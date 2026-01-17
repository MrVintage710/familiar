use std::path::PathBuf;

use tauri::ipc::InvokeError;
use thiserror::Error;

pub type FamiliarResult<T> = Result<T, FamiliarError>;
pub type FamiliarException = Result<(), FamiliarError>;

#[derive(Error, Debug)]
pub enum FamiliarError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    
    #[error("{0}")]
    FreError(#[from] fre::error::FreError),
    
    #[error("Invalid Rulebook: {0}. Make sure that the path is correct and that the file has the `.rulebook` extention.")]
    InvalidRulebook(PathBuf)
}

impl Into<InvokeError> for FamiliarError {
    fn into(self) -> InvokeError {
        InvokeError::from_error(self)
    }
}