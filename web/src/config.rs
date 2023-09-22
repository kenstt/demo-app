use std::env;

pub const ENV_LOG_LEVEL: &str = "LOG_LEVEL";
pub const ENV_APP_ENV: &str = "APP_ENV";

pub fn init() {
    let env = match env::var(ENV_APP_ENV) { // 取得環境變數的環境
        Ok(v) => v,
        _ => "dev".to_string(),
    };

    if env.starts_with("prod") { return; }  // 正式區則離開不讀.env檔

    match dotenvy::dotenv() {               // 讀取env檔案
        Ok(_) => println!(".env read successfully "),
        Err(e) => println!("Could not load .env file: {e}"),
    };
}