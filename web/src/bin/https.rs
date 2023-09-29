use web::{config, routers};
use service::logger::Logger;

#[tokio::main]        // warp::serve 方法是 Future，所以main要改成async
async fn main() {
    config::init();
    let _logger = Logger::builder()
        .use_env()
        // .add_package("hyper")        // 這裡可以各種試
        // .try_set_level("warn")       // builder的方法回傳self
        // .remove_package("hyper")     // 所以可以一直 chaining
        .build();                       // 最後再建立
    let routers = routers::all_routers();
    warp::serve(routers)
        .tls()
        .cert_path(config::tls_cert_path())
        .key_path(config::tls_key_path())
        .run(([0, 0, 0, 0], config::https_port()))
        .await;
}