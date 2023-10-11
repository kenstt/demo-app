use std::{env, ops::Add};
use chrono::{TimeZone, Utc};
use hmac::{Hmac, digest::{InvalidLength, KeyInit}};
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use warp::{
    Filter, Rejection, Reply,
    header::headers_cloned,
    http::{HeaderMap, HeaderValue}
};
use my_core::user:: Permission;
use my_core::user::{fake_query_user_permissions};
use crate::error::AppError;


pub fn login() -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json::<LoginRequest>())
        .and_then(login_handler)
}

pub async fn login_handler(req: LoginRequest) -> Result<impl Reply, Rejection> {
    let sub = req.username;
    if sub == "guest" {     // todo: 實作驗 user 帳密 (pass hash)
        return Err(Rejection::from(AppError::BadRequest("登入失敗".to_string())))
    }
    let permissions = fake_query_user_permissions(sub.clone());
    let permissions: Vec<u16> = permissions
        .iter()
        .map(|p| p.clone().into())
        .collect::<Vec<_>>();
    let exp = Utc::now()
        .add(chrono::Duration::hours(8))
        .timestamp();
    let claim = Claims {
        sub,
        exp,
        permissions,
    };
    let access_token = generate_jwt(key(), claim)?;
    let response = LoginResponse { access_token };
    Ok::<_, Rejection>(warp::reply::json(&response))
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

pub fn with_auth() -> impl Filter<Extract=(CurrentUser, ), Error=Rejection> + Clone {
    headers_cloned().and_then(|headers: HeaderMap<HeaderValue>| async move {
        let auth_header = headers.get("Authorization");
        match auth_header {
            None => {
                return Ok::<_, Rejection>(CurrentUser::Anonymous)
            }
            Some(auth_header) => {
                let token = auth_header.to_str().unwrap().to_string();
                let jwt = token.replace("Bearer ", "");
                let claims = verify_jwt(key(), jwt)?;
                let permissions = claims.permissions;
                let name = claims.sub;
                Ok::<_, Rejection>(CurrentUser::User { name, permissions })
            }
        }
    })
}

pub fn with_permission(permission: Permission) -> impl Filter<Extract=(), Error=Rejection> + Clone {
    with_auth().and_then(move |user: CurrentUser| {
        let p = permission.clone();
        async {
            let result = check_permission(user, p);
            match result {
                Ok(_) => {
                    Ok::<_, Rejection>(())
                }
                Err(_) => {
                    Err(warp::reject::custom(AppError::Unauthorized))
                }
            }
        }
    }).untuple_one()
}


pub fn check_permission(user: CurrentUser, permission: Permission) -> Result<(), Rejection> {
    let permission: u16 = permission.into();
    match user {
        CurrentUser::Anonymous => {
            Err(warp::reject::custom(AppError::Unauthorized))
        }
        CurrentUser::User { name, permissions } => {
            if permissions.iter().any(|&p| p == permission) {
                Ok::<_, Rejection>(())
            } else {
                Err(warp::reject::custom(AppError::Unauthorized))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum CurrentUser {
    Anonymous,
    User {
        name: String,
        permissions: Vec::<u16>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject
    /// https://tools.ietf.org/html/rfc7519#section-4.1.2
    pub sub: String,
    /// Expiration time (as UTC timestamp)
    /// the number of seconds (not milliseconds) since Epoch
    /// https://stackoverflow.com/a/39926886
    pub exp: i64,

    pub permissions: Vec<u16>,
}

/// 從環境變數取得JWT_SECRET作為簽章使用的KEY值
pub fn key() -> Hmac<Sha384> {
    let secret = env::var("JWT_SECRET").unwrap_or("some-secret".to_string());
    let key = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    key
}

/// 把字串轉換為加密用的Key物件
pub fn key_from_secret(secret: String) -> Result<Hmac<Sha384>, AppError> {
    let key = Hmac::new_from_slice(secret.as_bytes())?;
    Ok(key)
}

/// 產生JWT
pub fn generate_jwt(key: Hmac<Sha384>, claims: Claims) -> Result<String, AppError> {
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let token = Token::new(header, claims).sign_with_key(&key)?;

    Ok(token.as_str().to_string())
}

pub fn verify_jwt(key: Hmac<Sha384>, token: String) -> Result<Claims, AppError> {
    let verify: Result<Token<Header, Claims, _>, _> = token.verify_with_key(&key);
    match verify {
        Ok(token) => {
            let claims: Claims = token.claims().clone();
            let expiry = Utc.timestamp_opt(claims.exp, 0).unwrap();
            let now = Utc::now();
            if now > expiry {
                return Err(AppError::Unauthorized);
            }
            Ok(claims)
        }
        Err(_) => Err(AppError::Unauthorized),
    }
}

impl From<InvalidLength> for AppError {
    fn from(value: InvalidLength) -> Self {
        AppError::BadRequest(value.to_string())
    }
}

impl From<jwt::Error> for AppError {
    fn from(_value: jwt::Error) -> Self {
        AppError::Unauthorized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_jwt() {
        let key = key_from_secret("hello".to_string()).unwrap();
        let claims = Claims {
            sub: "someone".to_string(),
            // exp: 1000,
            exp: 3000000000, // 2065-01-24 05:20:00
            permissions: vec![],
        };
        let jwt = generate_jwt(key, claims).unwrap();
        // 1000 => 1970-01-01 00:00:01
        // assert_eq!(jwt, "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIiwiZXhwIjoxMDAwLCJwZXJtaXNzaW9ucyI6W119.Nf-IAcni3bO3W1c8lHeRr3B9zxuD9aJoheZIzacxWc8JpRId9WOMjAyy4va7ltpt");
        // 3000000000 => 2065-01-24 05:20:00
        assert_eq!(jwt, "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIiwiZXhwIjozMDAwMDAwMDAwLCJwZXJtaXNzaW9ucyI6W119.eu9ZQsu7GSUWEXugMD9AuRz_nuW23fk-16f4qGq4yJUSQFFWiAvQtLzK9lZN03Ef");
    }

    #[test]
    fn test_verify_expired_jwt_token() {
        let key = key_from_secret("hello".to_string()).unwrap();
        let token = "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIiwiZXhwIjoxMDAwfQ.fZNwRLGWCv9VsZYzU0jT3b-87SVFqFGlivzNeQCEOOLA1ARjAnoMPloyjs4_BEWt";

        let claim = verify_jwt(key, token.to_string());
        let error = claim.unwrap_err();
        assert_eq!(error, AppError::Unauthorized);
    }

    #[test]
    fn test_verify_valid_jwt_token() {
        let key = key_from_secret("hello".to_string()).unwrap();
        let token = "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIiwiZXhwIjozMDAwMDAwMDAwLCJwZXJtaXNzaW9ucyI6W119.eu9ZQsu7GSUWEXugMD9AuRz_nuW23fk-16f4qGq4yJUSQFFWiAvQtLzK9lZN03Ef";

        let claim = verify_jwt(key, token.to_string()).unwrap();
        assert_eq!(claim.sub, "someone");
        assert_eq!(claim.exp, 3000000000);
    }
}
