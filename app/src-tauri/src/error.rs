#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]  // 要能轉json
pub struct ErrorResponse {
    // 配合後端api的錯誤訊息建立
    message: String,
    details: Option<String>,
}

impl From<reqwest::Error> for ErrorResponse {
    // 加這個才能用語法糖?來簡化寫法
    fn from(value: reqwest::Error) -> Self {
        ErrorResponse {
            message: "主機連線失敗".into(),
            details: Some(value.to_string()),
        }
    }
}

impl From<service::tic_tac_toe::Error> for ErrorResponse {
    fn from(value: service::tic_tac_toe::Error) -> Self {
        match value {
            service::tic_tac_toe::Error::GameRules(msg) => ErrorResponse {
                message: "遊戲規則錯誤".into(),
                details: Some(msg),
            },
            service::tic_tac_toe::Error::GameOver => ErrorResponse {
                message: "遊戲結束".into(),
                details: None,
            },
            service::tic_tac_toe::Error::NotFound => ErrorResponse {
                message: "遊戲不存在".into(),
                details: None,
            },
            service::tic_tac_toe::Error::Unknown => ErrorResponse {
                message: "未知錯誤".into(),
                details: None,
            },
        }
    }
}

use tonic::Status;

impl From<Status> for ErrorResponse {           // 錯誤的mapping
    fn from(value: Status) -> Self {
        ErrorResponse {
            message: value.message().into(),    // Status.meesage為 &str
            details: Some(std::str::from_utf8(value.details())
                .unwrap_or_default()    // details是 &[u8]
                .to_string()),
        }
    }
}
