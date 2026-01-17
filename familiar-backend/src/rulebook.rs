use std::{collections::HashSet, fs::DirEntry, path::PathBuf};

use fre::rulebook::RulebookMeta;
use pak_db::meta::PakMeta;

use crate::error::{FamiliarError, FamiliarResult};


pub const RULEBOOK_DIR : &'static str = "./rulebooks";

#[tauri::command]
pub fn get_rulebook_list() -> FamiliarResult<Vec<PathBuf>> {
    fn check_file(entry : Result<DirEntry, std::io::Error>) -> FamiliarResult<Vec<PathBuf>> {
        let file = entry?;
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
    
    let paths : Vec<PathBuf> = std::fs::read_dir(RULEBOOK_DIR)?.filter_map(|path| check_file(path).ok()).flatten().collect();
    Ok(paths)
}

#[tauri::command]
pub fn get_meta_list() -> FamiliarResult<Vec<PakMeta>> {
    Ok(get_rulebook_list()?.into_iter()
        .filter_map(|path| PakMeta::read_meta(path).ok())
        .collect())
}

#[tauri::command]
pub fn get_available_game_systems() -> FamiliarResult<Vec<String>> {
    let mut game_systems = HashSet::<String>::new();
    
    get_meta_list()?
        .iter()
        .filter_map(|meta| meta.get_extra::<RulebookMeta>().ok())
        .for_each(|extra| { game_systems.insert(extra.game_system); });
    
    Ok(game_systems.into_iter().collect())
}