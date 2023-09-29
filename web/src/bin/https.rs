use web::config;
use web::logger;

fn main() {
    config::init();
    let _logger = logger::init();
    tracing::debug!("https server is starting");
}