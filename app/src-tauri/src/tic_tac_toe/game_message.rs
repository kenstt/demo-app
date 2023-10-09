use std::time::Duration;
use rand::random;
use tauri::Manager;
use tokio::time;

static mut IS_POLLING: bool = false;

#[tauri::command]
pub async fn polling_message(app: tauri::AppHandle) {
    tracing::info!("收到啟動訊息服務的指令，現在狀態為{}", status());
    unsafe {
        if IS_POLLING {                 // 避免重覆開啟服務
            return;                     // 若執行中則不再開
        }
        IS_POLLING = true;              // 設定註記為已開啟
    }
    tokio::spawn(async move {           // 開啟分身
        loop {
            let secs = random::<u64>() % 9_000 + 1_000; // 產生隨機等待豪秒
            time::sleep(Duration::from_millis(secs)).await;  // 等待

            unsafe {
                if !IS_POLLING {        // 若狀態為關閉
                    break;              // 則離開loop
                }                       // (關閉分身)
            }

            let message = my_core::game_message::message_factory();
            tracing::debug!("polling_message: {:?}", message);
            app.emit_all("message", message).unwrap();  // 發送事件
        }
    });
}

fn status() -> &'static str {
    unsafe {
        if IS_POLLING {
            "執行中。"
        } else {
            "停止。"
        }
    }
}

/// status的另一種寫法，但是不安全，呼叫者要放在unsafe block中
#[allow(dead_code)]
unsafe fn status_unsafe() -> &'static str { if IS_POLLING { "執行中。" } else { "停止。" } }

#[tauri::command]
pub async fn stop_polling_message() {
    tracing::info!("收到停止訊息服務的指令，現在狀態為{}", status());
    unsafe {
        IS_POLLING = false;
    }
}
