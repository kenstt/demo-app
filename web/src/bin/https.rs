use web::{config, routers, logger};

#[tokio::main]        // warp::serve 方法是 Future，所以main要改成async
async fn main() {
    config::init();
    let _logger = logger::init();
    let routers = routers::all_routers();
    warp::serve(routers)
        .tls()
        .cert_path(config::tls_cert_path())
        .key_path(config::tls_key_path())
        .run(([0, 0, 0, 0], config::https_port()))
        .await;
}