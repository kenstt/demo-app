use warp::{
    Filter, Rejection, Reply,
    cors::Builder,
    hyper::body::Bytes,
    path::FullPath,
    http::{HeaderMap, Method},
};
use std::net::SocketAddr;
use my_core::user::Permission;
use service::tic_tac_toe::TicTacToeService;
use crate::{error, tic_tac_toe};
use crate::app_context::AppContext;
use crate::open_api::api_doc_handler;
use crate::web_socket::ws_routers;
use crate::auth::{login, with_permission};

pub fn all_routers(ctx: AppContext)
    -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {

    let hello = warp::path("hello")
        .and(warp::get())
        .and(with_permission(Permission::Admin))
        .and(tracing())
        .map(|| {
            tracing::info!("saying hello...");
            "Hello, World!"
        });

    let static_files = warp::path("static")
        .and(warp::fs::dir("./static"));

    let game_service = service::tic_tac_toe::InMemoryTicTacToeService::new();
    game_service.new_game().unwrap();
    let api_games = tic_tac_toe::router_games(game_service);

    api_doc_handler()
        .or(hello)
        .or(login())
        .or(static_files)
        .or(ws_routers(ctx.clone()))
        .or(api_games)
        .recover(error::handle_rejection)
        .with(cors_config())
        .with(warp::trace::request())
}

fn cors_config() -> Builder {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "PUT", "POST", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"])
}

/// 注意：使用此API時必需要傳入http 的 query string，不然會報錯
fn tracing() -> impl Filter<Extract=(), Error=Rejection> + Clone {
    warp::addr::remote()
        .and(warp::header::headers_cloned())
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::query::raw())
        .and(warp::body::bytes())
        .and_then(|addr:Option<SocketAddr> , headers:HeaderMap, method:Method, path:FullPath, query: String, body:Bytes| async move {
            let query = query.to_string();
            let body = String::from_utf8(body.to_vec()).unwrap_or_default();
            tracing::warn!(
                "addr: {:?}\nmethod: {:?}\npath: {:?}\nquery: {:?}\nheaders: {:?}\nbody: {:?}",
                addr,
                method,
                path,
                query,
                headers,
                body
            );
            Ok::<(), Rejection>(())
        })
        .untuple_one()
}

// use futures_util::stream::StreamExt;
// use futures_util::FutureExt;
//
// pub fn ws_routers() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
//     warp::path("echo")
//         .and(warp::ws())
//         .map(|ws: warp::ws::Ws| {
//             ws.on_upgrade(|websocket| {
//                 let (tx, rx) = websocket.split();
//                 rx.forward(tx).map(|result| {
//                     if let Err(e) = result {
//                         tracing::info!("websocket error: {:?}", e);
//                     }
//                 })
//             })
//         })
// }
