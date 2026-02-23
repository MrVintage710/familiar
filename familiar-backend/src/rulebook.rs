use std::{collections::HashSet, fs::DirEntry, path::PathBuf};

use fre::{asset::Asset, constructor::Constructor, prelude::Rulebook, rulebook::{RulebookMeta, RulesetInfo}};
use pak_db::{meta::PakMeta, query::PakQuery};

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

pub fn get_rulebooks_in_rulesets(ruleset : &str) -> FamiliarResult<Vec<Rulebook>> {
    let rulebooks = get_rulebook_list()?
        .into_iter()
        .filter_map(|rulebook_path| Rulebook::from_file(rulebook_path).ok())
        .filter(|rulebook| {
            if rulebook.settings.ruleset.short().is_some() && rulebook.settings.ruleset.short().unwrap() == ruleset {return true}
            rulebook.settings.ruleset.title() == ruleset
        })
        .collect::<Vec<_>>()
    ;
    Ok(rulebooks)
}

#[tauri::command]
pub fn get_meta_list() -> FamiliarResult<Vec<PakMeta>> {
    Ok(get_rulebook_list()?.into_iter()
        .filter_map(|path| PakMeta::read_meta(path).ok())
        .collect())
}

#[tauri::command]
pub fn get_rulebook_settings() -> FamiliarResult<Vec<RulebookMeta>> {
    let list = get_meta_list()?
        .iter()
        .filter_map(|meta| meta.get_extra::<RulebookMeta>().ok())
        .collect::<Vec<_>>()
    ;
    
    Ok(list)
}

#[tauri::command]
pub fn get_available_rulesets() -> FamiliarResult<Vec<RulesetInfo>> {
    let mut game_systems = HashSet::<String>::new();
    let list = get_meta_list()?
        .iter()
        .filter_map(|meta| meta.get_extra::<RulebookMeta>().ok())
        .filter_map(|extra| {
            let title = extra.ruleset.title();
            if game_systems.contains(title) { return None }
            game_systems.insert(title.to_string());
            return Some(extra.ruleset);
        })
        .collect::<Vec<_>>()
    ;
    
    Ok(list)
}

#[tauri::command]
pub fn get_available_covers_for_ruleset(ruleset : &str) -> FamiliarResult<Vec<Asset>> {
    let covers = get_rulebooks_in_rulesets(ruleset)?
        .into_iter()
        .filter_map(|rulebook| rulebook.settings.cover)
        .collect::<Vec<_>>()
    ;
    
    Ok(covers)
}