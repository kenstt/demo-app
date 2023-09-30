use wasm_bindgen::prelude::*;
use service::tic_tac_toe::{InMemoryTicTacToeService, TicTacToeService};
use once_cell::sync::Lazy;
use crate::error::ErrorResponse;

static SERVICE: Lazy<InMemoryTicTacToeService> = Lazy::new(InMemoryTicTacToeService::new);

#[wasm_bindgen]
pub fn new_game() -> JsValue {
    let game = SERVICE.new_game().map_err(ErrorResponse::from);
    serde_wasm_bindgen::to_value(&game).unwrap()
}

#[wasm_bindgen]
pub fn get_game(id: usize) -> JsValue {
    let game = SERVICE.get(id).map_err(ErrorResponse::from);
    serde_wasm_bindgen::to_value(&game).unwrap()
}

#[wasm_bindgen]
pub fn play_game(id: usize, num: usize) -> JsValue {
    let game = SERVICE.play(id, num).map_err(ErrorResponse::from);
    serde_wasm_bindgen::to_value(&game).unwrap()
}

#[wasm_bindgen]
pub fn delete_game(id: usize) -> JsValue {
    let game = SERVICE.delete(id).map_err(ErrorResponse::from);
    serde_wasm_bindgen::to_value(&game).unwrap()
}