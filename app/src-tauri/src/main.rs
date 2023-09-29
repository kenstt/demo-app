// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod tic_tac_toe;
mod context;
mod logger;

use crate::context::Context;

use tic_tac_toe::rest_api::{get_game, new_game, play_game, delete_game};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    dotenvy::dotenv().ok();                     // 讀取環境變數.env
    let _logger = logger::init();
    tracing::info!("Starting tauri app");
    let context = Context::load();     // 初始化app共享物件
    tauri::Builder::default()
        .manage(context)    // 註冊為tauri的狀態物件
        .invoke_handler(tauri::generate_handler![
            greet,
            get_game, new_game, play_game, delete_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
