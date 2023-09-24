use warp::Filter;

mod logger;             // 抽出去的檔案
mod config;
mod tic_tac_toe;

use service::tic_tac_toe::TicTacToeService;

#[tokio::main]
async fn main() {
    config::init();
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

    let game_service = service::tic_tac_toe::InMemoryTicTacToeService::new();
    game_service.new_game().unwrap();     // db是空的，先製造一筆資料
    let api_games = tic_tac_toe::router_games(game_service);

    let routes = hello
        .or(api_games)
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
