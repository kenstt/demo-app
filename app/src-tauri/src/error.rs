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
pub struct ErrorResponse {                       // 配合後端api的錯誤訊息建立
message: String,
    details: Option<String>,
}

impl From<reqwest::Error> for ErrorResponse {    // 加這個才能用語法糖?來簡化寫法
fn from(value: reqwest::Error) -> Self {
    ErrorResponse {
        message: "主機連線失敗".into(),
        details: Some(value.to_string()),
    }
}
}
