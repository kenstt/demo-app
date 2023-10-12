pub mod hello_world;
pub mod tic_tac_toe;

use tonic::{transport::Server};
use tonic::transport::server::Router;
use hello_world::{greeter_server::GreeterServer, MyGreeter};
use tic_tac_toe::{tic_tac_toe_server::TicTacToeServer, TicTacToeGrpcService};

pub fn grpc_route() -> Router {
    let greeter = MyGreeter::default();
    let tic_tac_toe = TicTacToeGrpcService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(TicTacToeServer::new(tic_tac_toe))
}

