use warp::cors::Builder;
use warp::{Filter, Rejection, Reply};
use service::tic_tac_toe::TicTacToeService;
use crate::{error, tic_tac_toe};

pub fn all_routers() -> impl Filter<Extract= impl Reply, Error=Rejection> + Clone {
    let hello = warp::path("hello")
        .and(warp::get())
        .map(|| {
            tracing::info!("saying hello...");
            "Hello, World!"
        });

    let game_service = service::tic_tac_toe::InMemoryTicTacToeService::new();
    game_service.new_game().unwrap();
    let api_games = tic_tac_toe::router_games(game_service);

    hello
        .or(api_games)
        .recover(error::handle_rejection)
        .with(cors_config())
        .with(warp::trace::request())
}

fn cors_config() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "PUT", "POST", "DELETE"])
}
