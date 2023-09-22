use warp::Filter;

mod logger;             // 抽出去的檔案

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("./logs", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    logger::register_tracing(non_blocking);

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
