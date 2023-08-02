// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod expenses;

/// The main state struct of the application.
#[derive(Debug)]
pub struct State(Mutex<Vec<String>>); // TODO: Add proper state struct

fn main() {
    tauri::Builder::default()
        .manage(State(Mutex::new(vec![])))
        .invoke_handler(tauri::generate_handler![
            // commands here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
