// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::sync::Mutex;

mod data;
mod errors;

use data::{expenses, UserData};

/// The main state struct of the application.
#[derive(Debug, Deserialize)]
pub struct AppState(Mutex<UserData>);

fn main() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(UserData::new())))
        .invoke_handler(tauri::generate_handler![
            // commands here
            expenses::add_expenses,
            data::print_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
