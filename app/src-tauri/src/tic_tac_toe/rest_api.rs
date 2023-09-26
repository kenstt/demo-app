use crate::error::ErrorResponse;
use my_core::tic_tac_toe::Game;

#[tauri::command]
pub async fn new_game() -> Result<(isize, Game), ErrorResponse> {
    let url = format!("http://localhost:3030/tic_tac_toe");
    let client = reqwest::Client::new();
    let game = client.post(url).send().await?;
    unwrap_game(game).await
}

#[tauri::command]
pub async fn get_game(id: usize) -> Result<Game, ErrorResponse> {
    let url = format!("http://localhost:3030/tic_tac_toe/{}", id);
    let game = reqwest::get(url).await?;
    unwrap_game(game).await
}

#[tauri::command]
pub async fn play_game(id: usize, num: usize) -> Result<Game, ErrorResponse> {
    let url = format!("http://localhost:3030/tic_tac_toe/{id}/{num}");
    let client = reqwest::Client::new();
    let game = client.put(url).send().await?;
    unwrap_game(game).await
}

#[tauri::command]
pub async fn delete_game(id: usize) -> Result<(), ErrorResponse> {
    let url = format!("http://localhost:3030/tic_tac_toe/{id}");
    let client = reqwest::Client::new();
    client.delete(url).send().await?.text().await?;
    Ok(())
}

async fn unwrap_game<T>(game: reqwest::Response) -> Result<T, ErrorResponse>
    where T: serde::de::DeserializeOwned
{
    if game.status().is_success() {
        let game = game.json::<T>().await?;
        Ok(game)
    } else {
        let error = game.json::<ErrorResponse>().await?;
        Err(error)
    }
}