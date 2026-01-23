use fre::constructor::{Constructor};
use pak_db::{Pak, query::PakQuery};

mod rulebook;
mod error;

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
        .invoke_handler(tauri::generate_handler![greet, get_character_constructor, rulebook::get_available_game_systems])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
