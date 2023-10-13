use std::sync::Arc;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme}
};
use utoipa_swagger_ui::Config;
use utoipa_rapidoc::RapiDoc;
use warp::{
    Filter, Rejection, Reply,
    path::{FullPath, Tail},
    http::{Response, StatusCode, Uri}
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::tic_tac_toe::games_get,
        crate::tic_tac_toe::games_create,
        crate::tic_tac_toe::games_play,
        crate::tic_tac_toe::games_delete,
    ),
    components(
        schemas(
            my_core::tic_tac_toe::Game,
            my_core::tic_tac_toe::Symbol,
            crate::error::AppErrorMessage,
        )
    ),
    modifiers(& SecurityAddon),
    tags(
        (name = "game", description = "TicTacToe Game API"),
    ),
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "Authorization",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        )
    }
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}

pub fn api_doc_handler() -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let rapidoc_handler = warp::path("rapidoc")
        .and(warp::get())
        .map(|| warp::reply::html(RapiDoc::new("/api-doc.json").to_html()));

    let config = Arc::new(Config::from("/api-doc.json"));
    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    api_doc.or(rapidoc_handler).or(swagger_ui)
}

