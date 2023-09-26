use crate::error::Error;
use my_core::tic_tac_toe::Game;

#[tauri::command]               // 要給 WebView 呼叫，要加這個
pub async fn get_game(id: usize) -> Result<Game, Error> {
    let url = format!("http://localhost:3030/tic_tac_toe/{}", id);
    let game = reqwest::get(url)    // url 需要串網址放id，使用format組字串
        .await?                 // 這裡會得到 reqwest::Response 的結構體
        .json::<Game>()         // 把Response用json反序列化為Game結構體
        .await?;                // 解析Json是Future<完畢取得結果
    Ok(game)                    // 以上?會把Error外拋，若沒有，這裡會回傳game結構體
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