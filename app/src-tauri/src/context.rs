use std::env;
use std::sync::{Arc, RwLock};
use reqwest::{Client, Url};
use tonic::transport::Channel;

#[derive(Clone, Debug)]
pub struct Context {
    base_url: Url,          // base Url
    http_client: Client,    // 有連線池的http client
    channel: Channel,       // gRPC 用戶端物件
    pub token: Arc<RwLock<Option<String>>>,  // JWT token
}

impl Context {
    pub fn load() -> Self {
        let base_url = env::var("API_BASE_URL")
            .unwrap_or("http://localhost:3030/".to_string());
        let base_url = Url::parse(&base_url).unwrap();
        let http_client = Client::new();
        let grpc_url = env::var("GRPC_BASE_URL")            // 讀取環境變數
            .unwrap_or("http://[::1]:3032/".to_string());   // 預設值
        let channel: Channel = Channel::from_shared(grpc_url)
            .expect("需要設定正確的grpc url")
            .connect_lazy();           // 程式執行到這裡不主動連線，待下次需要用到才
        Self {
            base_url,
            http_client,
            channel,
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub fn base_url(&self) -> &Url {          // getter的概念
        &self.base_url
    }

    pub fn http_client(&self) -> &Client {    // getter
        &self.http_client
    }

    pub fn channel(&self) -> Channel { self.channel.clone() }   // getter

    pub fn token(&self) -> Option<String> {
        self.token.read().unwrap().clone()
    }
}
