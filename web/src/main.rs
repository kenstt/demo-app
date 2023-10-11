use std::net::IpAddr;
use std::str::FromStr;
use service::logger::Logger;
use web::{config, routers, app_context::AppContext};
use web::web_socket::polling_message;

#[tokio::main]
async fn main() {
    config::init();
    let _logger = Logger::builder().use_env().build();
    let app_context = AppContext::default();    // 加入App狀態機
    polling_message(&app_context).await;                // 加這行
    let routers = routers::all_routers(app_context.clone()); // 注入

    let addr_v6 = IpAddr::from_str("::0").unwrap();
    let addr_v4 = [0,0,0,0];
    tokio::join!(
        warp::serve(routers.clone()).run((addr_v4, config::http_port())),
        warp::serve(routers.clone())
            .tls()
            .cert_path(config::tls_cert_path())
            .key_path(config::tls_key_path())
            .run((addr_v4, config::https_port())),
        warp::serve(routers.clone()).run((addr_v6, 3036)),
        warp::serve(routers.clone())
            .tls()
            .cert_path(config::tls_cert_path())
            .key_path(config::tls_key_path())
            .run((addr_v6, 3037)),
    );
}
