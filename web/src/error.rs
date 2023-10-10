use std::convert::Infallible;
use std::error::Error;
use warp::{Rejection, Reply};
use warp::http::StatusCode;

#[derive(Debug, PartialEq)]
pub enum AppError {                  // 我們在這定義web專案可能會遇到的錯誤
    UserFriendly(String, String),    // 回傳訊息給予前端User使用
    BadRequest(String),              // 錯誤的要求
    NotFound(String),                // 找不到資源
    Unauthorized,                    // 未經授權的操作
    InternalServerError,             // 其他未歸類錯誤
}

#[derive(serde::Serialize)]
struct AppErrorMessage {              // 非 2XX 回應的Body
    message: String,                  // 錯誤的訊息內容
    details: Option<String>,          // 有關錯誤的細節資料（如果有的話）
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;                          // http 回應代碼
    let message;                      // http 回應訊息
    let mut details = None;          // http 回應細息

    if let Some(AppError::UserFriendly(msg, detail)) = err.find() {
        code = StatusCode::BAD_REQUEST;        // HTTP回應代碼
        message = msg.as_str();                // 所有message的賦值都要同一個類型，所以我們統一轉成 &str
        details = Some(detail.to_string());
    } else if let Some(AppError::BadRequest(msg)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = msg.as_str();
    } else if let Some(AppError::NotFound(msg)) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = msg.as_str();
    } else if let Some(AppError::Unauthorized) = err.find() {
        code = StatusCode::UNAUTHORIZED;
        message = "UNAUTHORIZED";
    } else if let Some(AppError::InternalServerError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else {
        tracing::error!("unhandled rejection: {:?}", err); // 把範例print改輸出至log
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";      // <= 字串值本身也是&str類別
    }

    let json = warp::reply::json(&AppErrorMessage {
        message: message.into(),
        details,
    });

    Ok(warp::reply::with_status(json, code))
}


type GameSrvError = service::tic_tac_toe::Error; // 定義給下面用的type。

impl From<GameSrvError> for AppError {
    fn from(value: GameSrvError) -> Self {
        match value {
            GameSrvError::GameRules(message) => AppError::UserFriendly("違反遊戲規則".into(), message),
            GameSrvError::GameOver => AppError::BadRequest("遊戲已結束".into()),
            GameSrvError::NotFound => AppError::NotFound("遊戲不存在".into()),
            GameSrvError::Unknown => AppError::InternalServerError,
        }
    }
}

impl warp::reject::Reject for AppError {}

use tonic::{Code, Status};

impl From<AppError> for Status {
    fn from(value: AppError) -> Self {
        match value {
            AppError::UserFriendly(e, m) => Status::with_details(Code::Aborted, e, m.into()),
            AppError::BadRequest(s) => Status::unavailable(s),
            AppError::NotFound(s) => Status::not_found(s),
            AppError::Unauthorized => Status::unauthenticated(""),
            AppError::InternalServerError => Status::unknown(""),
        }
    }
}