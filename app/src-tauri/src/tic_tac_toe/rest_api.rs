use crate::error::ErrorResponse;
use my_core::tic_tac_toe::Game;
use tauri::State;
use crate::context::Context;

#[tauri::command]
pub async fn new_game(ctx: State<'_, Context>) -> Result<(isize, Game), ErrorResponse> {
    let url = ctx.base_url().join("tic_tac_toe").unwrap();
    let game = ctx.http_client().post(url).send().await?;
    unwrap_game(game).await
}

#[tauri::command]
pub async fn get_game(id: usize, ctx: State<'_, Context>) -> Result<Game, ErrorResponse> {
    let url = ctx.base_url().join(&format!("tic_tac_toe/{}", id)).unwrap();
    let game = ctx.http_client().get(url).send().await?;
    unwrap_game(game).await
}

#[tauri::command]
pub async fn play_game(id: usize, num: usize, ctx: State<'_, Context>) -> Result<Game, ErrorResponse> {
    let url = ctx.base_url().join(&format!("tic_tac_toe/{}/{}", id, num)).unwrap();
    let game = ctx.http_client().put(url).send().await?;
    unwrap_game(game).await
}

#[tauri::command]
pub async fn delete_game(id: usize, ctx: State<'_, Context>) -> Result<(), ErrorResponse> {
    let url = ctx.base_url().join(&format!("tic_tac_toe/{}", id)).unwrap();
    ctx.http_client().delete(url).send().await?.text().await?;
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