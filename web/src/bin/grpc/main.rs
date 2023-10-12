use tonic::{transport::Server};
use web::grpc::hello_world::{
    greeter_server::GreeterServer,    // build 幫我們產生的 gRPC Server
    MyGreeter,                        // 我們實作的方法
};
use web::grpc::tic_tac_toe::{tic_tac_toe_server::TicTacToeServer, TicTacToeGrpcService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = service::logger::Logger::builder()
        .add_package("grpc")
        .add_package("tonic")
        .build();
    let addr = "[::1]:3032".parse().unwrap();        // 監聽的addr & port
    let greeter = MyGreeter::default();              // 我們實作的服務
    let tic_tac_toe = TicTacToeGrpcService::default();

    tracing::info!("GreeterServer listening on {}", addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))    // 加入gRPC服務
        .add_service(TicTacToeServer::new(tic_tac_toe))
        .serve(addr)
        .await?;

    Ok(())
}