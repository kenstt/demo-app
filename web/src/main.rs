use web::{config, logger, routers};

#[tokio::main]
async fn main() {
    config::init();
    let _logger = logger::init();
    let routers = routers::all_routers();
    warp::serve(routers).run(([127, 0, 0, 1], 3030)).await;
}
