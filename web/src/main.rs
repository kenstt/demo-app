use web::{config, logger, routers};

#[tokio::main]
async fn main() {
    config::init();
    let _logger = logger::init();
    let routers = routers::all_routers();
    warp::serve(routers).run(([0, 0, 0, 0], config::http_port())).await;
}
