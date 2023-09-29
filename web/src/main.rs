use service::logger::Logger;
use web::{config, routers};

#[tokio::main]
async fn main() {
    config::init();
    let _logger = Logger::builder().use_env().build();
    let routers = routers::all_routers();
    warp::serve(routers).run(([0, 0, 0, 0], config::http_port())).await;
}
