// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod tic_tac_toe;
mod context;
mod hello_grpc;

use crate::hello_grpc::say_hello;
use crate::context::Context;
use service::logger::Logger;

use tic_tac_toe::rest_api::{get_game, new_game, play_game, delete_game};
use service::tic_tac_toe::InMemoryTicTacToeService;
use tic_tac_toe::embedded::{get_game_e, new_game_e, play_game_e, delete_game_e};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]    // 把原本的 main 改成 async main
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();                     // 讀取環境變數.env
    let _logger = service::logger::Logger::builder().use_env().build();
    tracing::info!("Starting tauri app");
    say_hello().await;
    let context = Context::load();     // 初始化app共享物件
    let game_service = InMemoryTicTacToeService::new(); // 建立 在tauri執行的service
    tauri::Builder::default()
        .manage(context)    // 註冊為tauri的狀態物件
        .manage(game_service)        // 註冊game_service服務
        .invoke_handler(tauri::generate_handler![
            greet,
            get_game, new_game, play_game, delete_game,
            get_game_e, new_game_e, play_game_e, delete_game_e,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
