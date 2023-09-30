use tauri::State;
use my_core::tic_tac_toe::Game;
use service::tic_tac_toe::{InMemoryTicTacToeService, TicTacToeService};
use crate::error::ErrorResponse;

#[tauri::command]    // 這是個讓讓前端呼叫的指令
pub async fn new_game_e(srv: State<'_, InMemoryTicTacToeService>)
                        -> Result<(usize, Game), ErrorResponse> {
    let game = srv.new_game()?; // 直接把 service呼叫的結果回傳。
    Ok(game)
}

#[tauri::command]
pub async fn get_game_e(id: usize, srv: State<'_, InMemoryTicTacToeService>)
                        -> Result<Game, ErrorResponse> {
    let game = srv.get(id)?;
    Ok(game)
}

#[tauri::command]
pub async fn play_game_e(id: usize, num: usize, srv: State<'_, InMemoryTicTacToeService>)
                         -> Result<Game, ErrorResponse> {
    let game = srv.play(id, num)?;
    Ok(game)
}

#[tauri::command]
pub async fn delete_game_e(id: usize, srv: State<'_, InMemoryTicTacToeService>)
                           -> Result<(), ErrorResponse> {
    srv.delete(id)?;
    Ok(())
}