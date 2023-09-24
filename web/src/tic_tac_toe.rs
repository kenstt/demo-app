use warp::{Filter, Rejection};
use service::tic_tac_toe::TicTacToeService;
use crate::error::AppError;
use warp::http::StatusCode;

pub fn router_games(
    service: impl TicTacToeService
) -> impl Filter<Extract=(impl warp::Reply, ), Error=Rejection> + Clone {
    games_get(service.clone())
        .or(games_create(service.clone()))
        .or(games_play(service.clone()))
        .or(games_delete(service))
}

/// GET /tic_tac_toe/:id
pub fn games_get(
    service: impl TicTacToeService
) -> impl Filter<Extract=(impl warp::Reply, ), Error=Rejection> + Clone {
    // 這邊想用path傳id，是參考todo範例的delete怎麼傳path參數
    warp::path!("tic_tac_toe" / usize)                 // 改成我們想要的路由
        .and(warp::get())                              // 設為 get 方法
        .and(warp::any().map(move || service.clone())) // 仿範例with_db寫法
        // .and(with_service(service))
        .and_then(handle_games_get)
}

// 保留warp 中 example寫法參考
// fn with_service(service: impl TicTacToeService) -> impl Filter<Extract=(impl TicTacToeService, ), Error=std::convert::Infallible> + Clone {
//     warp::any().map(move || service.clone())
// }

pub async fn handle_games_get(
    id: usize,
    service: impl TicTacToeService,
) -> Result<impl warp::Reply, Rejection> {
    let game = service.get(id).map_err(AppError::from)?;
    Ok(warp::reply::json(&game))
}

/// POST /tic_tac_toe/
pub fn games_create(
    service: impl TicTacToeService
) -> impl Filter<Extract=(impl warp::Reply, ), Error=Rejection> + Clone {
    warp::path!("tic_tac_toe" )
        .and(warp::post())
        .and_then(move || {
            let service = service.clone();
            async move {
                let games = service.new_game().map_err(AppError::from)?;
                Ok::<_, Rejection>(warp::reply::json(&games))
            }
        })
}

/// PUT /tic_tac_toe/:id/:num
pub fn games_play(
    service: impl TicTacToeService
) -> impl Filter<Extract=(impl warp::Reply, ), Error=Rejection> + Clone {
    warp::path!("tic_tac_toe" / usize / usize)
        .and(warp::put())
        .and_then(move |id, num| {
            let service = service.clone();
            async move {
                let games = service.play(id, num).map_err(AppError::from)?;
                Ok::<_, Rejection>(warp::reply::json(&games))
            }
        })
}

/// DELETE /tic_tac_toe/:id
pub fn games_delete(
    service: impl TicTacToeService
) -> impl Filter<Extract=(impl warp::Reply, ), Error=Rejection> + Clone {
    warp::path!("tic_tac_toe" / usize)
        .and(warp::delete())
        .and_then(move |id| {
            let service = service.clone();
            async move {
                service.delete(id).map_err(AppError::from)?;
                Ok::<_, Rejection>(StatusCode::NO_CONTENT)
            }
        })
}

