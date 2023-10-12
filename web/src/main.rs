use std::net::IpAddr;
use std::str::FromStr;
use service::logger::Logger;
use web::{config, routers, app_context::AppContext};
use web::grpc::grpc_route;
use web::web_socket::polling_message;

#[tokio::main]
async fn main() {
    config::init();
    let _logger = Logger::builder().use_env().add_package("grpc").add_package("tonic").build();
    let app_context = AppContext::default();
    polling_message(&app_context).await;
    let routers = routers::all_routers(app_context.clone());

    let addr_v6 = IpAddr::from_str("::0").unwrap();
    let addr_v4 = [0,0,0,0];

    let grpc_addr = "[::1]:3032".parse().unwrap();

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
        grpc_route().serve(grpc_addr),
    );
}
