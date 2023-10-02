pub async fn hello_async() -> String {
    // wait().await;               // 加這行在執行時會失效
    "Hello async!".to_string()
}

pub async fn wait() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}