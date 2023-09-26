use crate::error::{Error, ErrorResponse};
use my_core::tic_tac_toe::Game;

#[tauri::command]               // 要給 WebView 呼叫，要加這個
pub async fn get_game(id: usize) -> Result<Game, ErrorResponse> {
    let url = format!("http://localhost:3030/tic_tac_toe/{}", id);
    let game = reqwest::get(url).await?;      // Response
    if game.status().is_success() {           // 2xx的代碼代表成功
        let game = game.json::<Game>().await?;
        Ok(game)       // 成功回傳 game 物件
    } else {
        let error = game.json::<ErrorResponse>().await?;
        Err(error)     // 失敗則解析後端api回傳的body訊息再傳給前端js
    }
}

#[tauri::command]
pub async fn new_game() -> Result<(isize, Game), Error> {
    let url = format!("http://localhost:3030/tic_tac_toe");
    let client = reqwest::Client::new();    // 建立 client
    let game = client.post(url)        // 設定post及網址
        .send()                        // 送出request要求
        .await?                        // await取得 Result<Response, Error>
        .json::<(isize, Game)>()       // 維持原API的輸出讓前端API介面一致
        .await?;                       // Result<(isize, Game)>,

    Ok(game)
}