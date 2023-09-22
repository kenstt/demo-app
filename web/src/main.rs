use warp::Filter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    let file_appender = tracing_appender::rolling::daily("./logs", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(tracing_filter())
        .with(tracing_subscriber::fmt::layer())  // 輸出至終端
        .with(tracing_subscriber::fmt::layer()
            .with_writer(non_blocking))          // 輸出至檔案
        .init();

    let hello = warp::path("hello")
        .and(warp::get())
        .map(|| {
            tracing::info!("saying hello...");
            "Hello, World!"
        })
        .with(warp::trace::named("hello"));

    let routes = hello
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub fn tracing_filter() -> EnvFilter {
    EnvFilter::from_default_env()
        .add_directive("warp=debug".parse().unwrap())
        .add_directive("web=debug".parse().unwrap())
        .add_directive("tracing=debug".parse().unwrap())
}