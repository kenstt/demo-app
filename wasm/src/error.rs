#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub details: Option<String>,
}

type SrvError = service::tic_tac_toe::Error; // 簡化下面SrvError長度
// ↑不直接 use service::tic_tac_toe::Error 是怕之後要做Error會命名衝突
impl From<SrvError> for ErrorResponse {
    fn from(value: SrvError) -> Self {
        match value {
            SrvError::GameRules(msg) => ErrorResponse {
                message: "違反遊戲規則".to_string(),
                details: Some(msg),
            },
            SrvError::GameOver => ErrorResponse {
                message: "遊戲已結束".to_string(),
                details: None,
            },
            SrvError::NotFound => ErrorResponse {
                message: "遊戲不存在".to_string(),
                details: None,
            },
            SrvError::Unknown => ErrorResponse {
                message: "未知錯誤".to_string(),
                details: None,
            },
        }
    }
}