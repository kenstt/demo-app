use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn tracing_filter() -> EnvFilter {
    EnvFilter::from_default_env()
        .add_directive("warp=debug".parse().unwrap())
        .add_directive("web=debug".parse().unwrap())
        .add_directive("tracing=debug".parse().unwrap())
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