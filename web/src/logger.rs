use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::ENV_LOG_LEVEL;

pub fn tracing_filter() -> EnvFilter {
    let level = std::env::var(ENV_LOG_LEVEL).unwrap_or_else(|_| "info".to_string());
    let dir1 = format!("warp={}", level);
    let dir2 = format!("web={}", level);
    let dir3 = format!("tracing={}", level);
    EnvFilter::from_default_env()
        .add_directive(dir1.parse().unwrap())
        .add_directive(dir2.parse().unwrap())
        .add_directive(dir3.parse().unwrap())
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