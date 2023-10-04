tonic::include_proto!("game");                // 引入自動建立的代碼

use web::error::AppError;
use tonic::{Request, Response, Status};
use tic_tac_toe_server::{TicTacToe};
use service::tic_tac_toe::{InMemoryTicTacToeService, TicTacToeService};

type CoreGame = my_core::tic_tac_toe::Game;
// 別名：識別core裡的Game
type CoreSymbol = my_core::tic_tac_toe::Symbol;    // 別名：識別core裡的Symbol

pub struct TicTacToeGrpcService {
    service: InMemoryTicTacToeService,
}

impl Default for TicTacToeGrpcService {
    fn default() -> Self {
        Self {
            service: InMemoryTicTacToeService::new()
        }
    }
}

#[tonic::async_trait]                         // rust原生不支援async trait
impl TicTacToe for TicTacToeGrpcService {
    async fn get_game(
        &self, request: Request<IdRequest>,
    ) -> Result<Response<Game>, Status> {
        let id = request.into_inner().id.try_into().unwrap();
        let game = self.service.get(id).map_err(AppError::from)?;
        Ok(Response::new(game.into()))
    }

    async fn play(
        &self, request: Request<PlayRequest>,
    ) -> Result<Response<Game>, Status> {
        let req = request.into_inner();     // play 的請求有兩個參數
        let game = self.service.play(
            req.id.try_into().unwrap(),
            req.num.try_into().unwrap(),
        ).map_err(AppError::from)?;
        Ok(Response::new(game.into()))
    }

    async fn new_game(
        &self, _request: Request<EmptyRequest>,
    ) -> Result<Response<GameSet>, Status> {
        let (id, game) = self.service.new_game().map_err(AppError::from)?;
        let game_set = GameSet {        // gRPC產的結構體GameSet
            id: id.try_into().unwrap(),                         // id: id的縮寫
            game: Some(game.into()),    // 一樣用 into ，配合類別要放在Some裡
        };
        Ok(Response::new(game_set))
    }

    async fn delete_game(
        &self, request: Request<IdRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let id = request.into_inner().id;
        self.service.delete(id.try_into().unwrap()).map_err(AppError::from)?;
        Ok(Response::new(EmptyResponse::default()))        // 無回傳值我們給空白物件
    }
}

// 實作From trait
impl From<CoreGame> for Game {
    fn from(game: CoreGame) -> Self {
        Self {
            cells: game.cells.iter()              // gRPC 的 cell 是 Vec<i32>
                .map(|&x| match x {               // map CoreGame to grpc
                    None => 2,                    // gRPC enum 的 2
                    Some(sym) => match sym {
                        CoreSymbol::O => 0,       // gRPC enum 的 0
                        CoreSymbol::X => 1,       // gRPC enum 的 1
                    },
                }).collect()
            ,
            is_over: game.is_over,                // boolean沒什麼問題
            winner: match game.winner {           // 類別為 Option<i32>
                None => None,                     // optional可為None
                Some(sym) => match sym {
                    CoreSymbol::O => Some(0),     // Option 的值要用 Some 包起來
                    CoreSymbol::X => Some(1),
                }
            },
            won_line: match game.won_line {       // 類別為 Vec<u32>
                None => vec![],                   // 給空白清單
                Some(x) => x.into(),              // 利用into幫我們把Array轉Vec
            },
        }
    }
}