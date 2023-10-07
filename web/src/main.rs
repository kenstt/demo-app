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
    warp::serve(routers).run(([0, 0, 0, 0], config::http_port())).await;
}
