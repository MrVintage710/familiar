use std::{str::FromStr, sync::Mutex};

use fre::{common::{choice::Input, meta::HasItemMeta}, constructor::Constructor};
use mlua::Lua;
use pak_db::query::PakQuery;
use tauri::State;
use uuid::Uuid;

use crate::{cache::Cache, error::{FamiliarError, FamiliarException, FamiliarResult}, rulebook::get_rulebooks_in_rulesets};

//==============================================================================================
//        Constructor API calls
//==============================================================================================

#[tauri::command]
pub fn constructor_get_all_from_ruleset(ruleset : &str) -> FamiliarResult<Vec<Constructor>> {
    let constructors = get_rulebooks_in_rulesets(ruleset)?
        .into_iter()
        .filter_map(|rulebook| rulebook.pak.query::<(Constructor, )>(PakQuery::All).ok())
        .flatten()
        .collect::<Vec<_>>()
    ;
    
    Ok(constructors)
}

//==============================================================================================
//        Cached Constructor Calls
//==============================================================================================


#[tauri::command]
pub fn constructor_cache_character_creator(ruleset : &str, cache : State<'_, Mutex<Cache<Constructor>>>) -> FamiliarResult<String> {
    let mut cache = cache.lock().unwrap();
    let Some(character_constructor) = constructor_get_all_from_ruleset(ruleset)?
        .into_iter()
        .find(|constructor| constructor.get_meta().has_tag("Character Creator"))
    else { return Err(FamiliarError::missing_character_constructor(ruleset)) };
    
    let uuid = character_constructor.get_meta().uuid();
    
    cache.cache(character_constructor);
 
    println!("Caching Constructor: `{uuid}`");
    return Ok(uuid.to_string())
}

#[tauri::command]
pub fn constructor_get_step(uuid : &str, index : usize, cache : State<'_, Mutex<Cache<Constructor>>>) -> FamiliarResult<Input> {
    let uuid = Uuid::from_str(uuid)?;
    let cache = cache.lock().unwrap();
    let Some(character_constructor) = cache.get(&uuid) else {return Err(FamiliarError::cache_object_not_found("Constructor", &uuid))};
    
    let input = character_constructor.process_step(&Lua::new(), index)?;
    
    return Ok(input);
}

#[tauri::command]
pub fn constructor_release_ref(uuid : &str, cache : State<'_, Mutex<Cache<Constructor>>>) -> FamiliarException {
    let uuid = Uuid::from_str(uuid)?;
    let mut cache = cache.lock().unwrap();
    cache.decrement_count(&uuid);
    Ok(())
}