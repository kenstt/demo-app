use std::net::IpAddr;
use std::str::FromStr;
use web::{config, routers};
use service::logger::Logger;
use web::app_context::AppContext;
use web::web_socket::polling_message;

#[tokio::main]        // warp::serve 方法是 Future，所以main要改成async
async fn main() {
    config::init();
    let _logger = Logger::builder()
        .use_env()
        // .add_package("hyper")        // 這裡可以各種試
        // .try_set_level("warn")       // builder的方法回傳self
        // .remove_package("hyper")     // 所以可以一直 chaining
        .build();                       // 最後再建立
    let app_context = AppContext::default();    // 加入App狀態機
    polling_message(&app_context).await;                // 加這行
    let routers = routers::all_routers(app_context.clone()); // 注入
    let addr = IpAddr::from_str("::0").unwrap();
    warp::serve(routers)
        .tls()
        .cert_path(config::tls_cert_path())
        .key_path(config::tls_key_path())
        .run((addr, config::https_port()))
        .await;
}