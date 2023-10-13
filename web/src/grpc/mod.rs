pub mod hello_world;
pub mod tic_tac_toe;

use tonic::{Request, Status, transport::Server};
use tonic::transport::server::Router;
use hello_world::{greeter_server::GreeterServer, MyGreeter};
use tic_tac_toe::{tic_tac_toe_server::TicTacToeServer, TicTacToeGrpcService};
use crate::auth::{CurrentUser, key, verify_jwt};

pub fn grpc_route() -> Router {
    let greeter = MyGreeter::default();
    let tic_tac_toe = TicTacToeGrpcService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(TicTacToeServer::with_interceptor(tic_tac_toe, with_auth))
}

fn with_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(token) => {
            let jwt = token.to_str().unwrap().to_string();
            let jwt = jwt.replace("Bearer ", "");
            tracing::info!("token: {}", jwt);
            let claims = verify_jwt(key(), jwt)?;
            let permissions = claims.permissions;
            let name = claims.sub;
            let user = CurrentUser::User { name, permissions };
            req.extensions_mut().insert(user);
        },
        _ => {
            req.extensions_mut().insert(CurrentUser::Anonymous);
        },
    }
    Ok(req)
}
