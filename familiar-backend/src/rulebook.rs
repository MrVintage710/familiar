use std::{collections::HashSet, path::PathBuf};

use fre::error::FreError;
use pak_db::meta::PakMeta;

use crate::error::{FamiliarError, FamiliarResult};


pub const RULEBOOK_DIR : &'static str = "./rulebooks/";

pub fn get_rulebook_list() -> FamiliarResult<Vec<PathBuf>> {
    fn check_file(entry : Result<DirEntry, Error>) -> FamiliarResult<Vec<PathBuf>> {
        let file = file?;
        let file_type = file.file_type()?;
        let path = file.path();
        if file_type.is_file() {
            let Some(extention) = path.extension() else { return Err(FamiliarError::InvalidRulebook(path)) };
            if extention != "rulebook" { return Err(FamiliarError::InvalidRulebook(path)) }
            Ok(vec![path])
        } else {
            Ok(std::fs::read_dir(path)?.filter_map(|path| check_file(path).ok()).flatten().collect())
        }
    }
    
    let paths : Vec<PathBuf> = std::fs::read_dir(path)?.filter_map(|path| check_file(path).ok()).flatten().collect();
    Ok(paths)
}

pub fn get_meta_list() -> FamiliarResult<Vec<PakMeta>> {
    Ok(get_rulebook_list()?.into_iter()
        .map(|path| -> ))
}

#[tauri::command]
pub fn get_available_game_systems() {
    let game_systems = HashSet::<String>::new();
    
}