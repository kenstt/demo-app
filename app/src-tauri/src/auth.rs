use serde::{Deserialize, Serialize};
use tauri::State;
use crate::context::Context;
use crate::error::ErrorResponse;

#[tauri::command]
pub async fn login(
    username: String,
    password: String,
    mut ctx: State<'_, Context>
) -> Result<LoginResponse, ErrorResponse> {
    let url = ctx
        .base_url()
        .join("login")
        .unwrap();
    let response = ctx
        .http_client()
        .post(url)
        .json(&LoginRequest {
            username,
            password,
        })
        .send()
        .await?;
    let jwt: LoginResponse = response
        .json()
        .await?;
    let mut token = ctx.token
        .write()
        .unwrap();
    *token = Some(jwt.clone().access_token);
    tracing::info!("ctx: {:?}", ctx.token);
    Ok(jwt)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}