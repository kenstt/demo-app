use warp::{Filter, Rejection};
use service::tic_tac_toe::TicTacToeService;
use crate::error::AppError;

pub fn router_games(
    service: impl TicTacToeService          // 把範例db改成我們的service
) -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    games_get(service.clone())
    // .or(games_create(db.clone()))        // 先註解，保留等等擴充
    // .or(games_update(db.clone()))
    // .or(games_delete(db))
}

/// GET /tic_tac_toe/:id
pub fn games_get(
    service: impl TicTacToeService
) -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
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
    let game = service.get(id).map_err(AppError::from)?;;
    Ok(warp::reply::json(&game))
}