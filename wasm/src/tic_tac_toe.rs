use service::tic_tac_toe::{InMemoryTicTacToeService, TicTacToeService};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;  // 引入wasm_bindgen的東西

static SERVICE: Lazy<InMemoryTicTacToeService> = Lazy::new(InMemoryTicTacToeService::new);


use my_core::tic_tac_toe::Game;    // 等等就移除了
use service::tic_tac_toe::Error;   // 等等就移除了+1

#[wasm_bindgen]        // 幫我們整成給wasm 呼叫的api的巨集
pub fn new_game() -> Result<Game, Error> {
    SERVICE.new_game()
}
