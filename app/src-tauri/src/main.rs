// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod tic_tac_toe;

use tic_tac_toe::rest_api::{get_game, new_game};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_game, new_game, // 這裡加入我們剛剛寫的api client
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
