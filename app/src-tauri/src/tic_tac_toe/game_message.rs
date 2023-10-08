use std::time::Duration;
use rand::random;
use tauri::Manager;
use tokio::time;

#[tauri::command]
pub async fn polling_message(app: tauri::AppHandle) {
    tokio::spawn(async move {
        loop {
            let secs = random::<u64>() % 9_000 + 1_000; // 產生隨機等待豪秒
            time::sleep(Duration::from_millis(secs)).await;  // 等待

            let message = my_core::game_message::message_factory();
            tracing::debug!("polling_message: {:?}", message);
            app.emit_all("message", message).unwrap();
        }
    });
}