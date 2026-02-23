use std::sync::Mutex;

use fre::constructor::{Constructor};
use pak_db::{Pak, query::PakQuery};
use tauri::Manager;

use crate::{cache::Cache};

mod rulebook;
mod error;
mod constructor;
mod cache;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn test_character(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_character_constructor() -> Vec<Constructor> {
    let pak = Pak::new_from_file("./rulebook.pak").unwrap();
    let constructors = pak.query::<(Constructor, )>(PakQuery::All).unwrap();
    println!("{constructors:?}");
    constructors
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_character_constructor, 
            rulebook::get_available_rulesets, 
            rulebook::get_available_covers_for_ruleset,
            rulebook::get_rulebook_settings,
            
            constructor::constructor_get_all_from_ruleset,
            constructor::constructor_get_step,
            constructor::constructor_cache_character_creator,
            constructor::constructor_release_ref,
        ])
        .setup(|app| {
            app.manage(Mutex::new(Cache::<Constructor>::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
