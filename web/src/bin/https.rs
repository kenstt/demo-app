use web::{config, routers, logger};

#[tokio::main]        // warp::serve 方法是 Future，所以main要改成async
async fn main() {
    config::init();
    let _logger = logger::init();
    let routers = routers::all_routers();
    warp::serve(routers).run(([127, 0, 0, 1], 3031)).await;
}