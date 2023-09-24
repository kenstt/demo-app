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

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::request;
    use service::tic_tac_toe::InMemoryTicTacToeService;
    use crate::error::handle_rejection;
    use my_core::tic_tac_toe::Game;

    #[tokio::test]
    async fn test_games_get() {
        let service = InMemoryTicTacToeService::new();
        let (id, _) = service.new_game().unwrap();
        let api = games_get(service);

        let res = request()
            .method("GET")
            .path(&format!("/tic_tac_toe/{}", id))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let body = res.into_body();    // 取得body的Bytes
        let game: Game = serde_json::from_slice(&body).unwrap(); // 反序列化為物件
        assert_eq!(game.is_over, false);
        assert_eq!(game.winner, None);
        let is_empty = game.cells.iter().all(|x| *x == None);
        assert_eq!(is_empty, true);
    }

    #[tokio::test]
    async fn test_games_create() {
        let service = InMemoryTicTacToeService::new();
        let api = games_create(service);

        let res = request()
            .method("POST")
            .path("/tic_tac_toe")
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_games_play() {
        let service = InMemoryTicTacToeService::new();
        let (id, _) = service.new_game().unwrap();
        let api = games_play(service);

        let res = request()
            .method("PUT")
            .path(&format!("/tic_tac_toe/{}/{}", id, 1))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_games_delete() {
        let service = InMemoryTicTacToeService::new();
        let (id, _) = service.new_game().unwrap();
        let api = games_delete(service);

        let res = request()
            .method("DELETE")
            .path(&format!("/tic_tac_toe/{}", id))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_games_delete_not_found() {
        let service = InMemoryTicTacToeService::new();
        let api = games_delete(service)
            .recover(handle_rejection);    // 記得加error handling

        let res = request()               // 沒加我們寫的handle_rejection會變500
            .method("DELETE")
            .path(&format!("/tic_tac_toe/{}", 12))
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_games_play_non_empty() {
        let service = InMemoryTicTacToeService::new();
        let (id, _) = service.new_game().unwrap();
        let api = games_play(service.clone()).recover(handle_rejection);
        service.play(id, 1).unwrap();     // 模擬已下第一格

        let res = request()
            .method("PUT")
            .path(&format!("/tic_tac_toe/{}/{}", id, 1)) // 重複下第一格 應報錯
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }
}