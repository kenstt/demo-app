use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("./logs", "log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    register_tracing(non_blocking);
    guard
}

pub fn tracing_filter() -> EnvFilter {
    let level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let dir1 = format!("tauri={}", level);
    let dir2 = format!("app={}", level);
    let dir3 = format!("tracing={}", level);
    let dir4 = format!("reqwest={}", level);    // 加上我們執行檔https的名字
    EnvFilter::from_default_env()
        .add_directive(dir1.parse().unwrap())
        .add_directive(dir2.parse().unwrap())
        .add_directive(dir3.parse().unwrap())
        .add_directive(dir4.parse().unwrap())
}

pub fn register_tracing(non_blocking: NonBlocking) {
    tracing_subscriber::registry()
        .with(tracing_filter())
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(non_blocking))
        .init()
}