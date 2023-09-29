use warp::Filter;
use web::{config, error, tic_tac_toe, logger};
use service::tic_tac_toe::TicTacToeService;

#[tokio::main]
async fn main() {
    config::init();
    let _logger = logger::init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "PUT", "POST", "DELETE"]);

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
        .recover(error::handle_rejection)
        .with(cors)
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
