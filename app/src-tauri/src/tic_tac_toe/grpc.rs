tonic::include_proto!("game");        // 引入 proto 產生的rust資料

type CoreGame = my_core::tic_tac_toe::Game;        // 避免與gRPC的Game名稱衝突
type CoreSymbol = my_core::tic_tac_toe::Symbol;    // 避免與gRPC的Symbol名稱衝突

impl From<Game> for CoreGame {                     // mapping
    fn from(value: Game) -> Self {
        CoreGame {
            cells: value.cells                     // Vec<i32>
                .iter()
                .map(|x| match x {
                    0 => Some(CoreSymbol::O),      // gRPC symbol enum O = 0
                    1 => Some(CoreSymbol::X),      // gRPC symbol enum X = 1
                    _ => None
                })
                .collect::<Vec<_>>()          // Vec<Option<Symbol>, Global>
                .try_into()              // Result<[Option<...>;9], Vec<..>>
                .unwrap(),
            is_over: value.is_over,
            winner: match value.winner {        // Option<i32>
                None => None,
                Some(x) => match x {
                    0 => Some(CoreSymbol::O),   // i32 轉 rust enum
                    1 => Some(CoreSymbol::X),   // i32 轉 rust enum
                    _ => None
                }
            },
            won_line: {
                if value.won_line.is_empty() {  // Vec<u32>
                    None
                } else {
                    Some(value.won_line.try_into().unwrap())
                }
            },
        }
    }
}


use tauri::State;
use tonic::{Request, metadata::MetadataValue};
use crate::context::Context;
use crate::error::ErrorResponse;
use tic_tac_toe_client::TicTacToeClient;        // 使用proto產出的 Client

#[tauri::command]
pub async fn new_game_grpc(ctx: State<'_, Context>)     // 注入Context
    -> Result<(u32, CoreGame), ErrorResponse> {
    let channel = ctx.channel();                        // 取得連線池channel
    let token = ctx.token();
    let mut client = TicTacToeClient::with_interceptor(channel,
        move |mut req: Request<()>| {
            tracing:: info!("info: {:?}", token);
            if token.is_some() {
                let jwt = token.clone().unwrap();
                let bearer: MetadataValue<_> = format!("Bearer {}", jwt).parse().unwrap();
                req.metadata_mut().insert("authorization", bearer);
            }
            Ok(req)
        });
    let request = tonic::Request::new(EmptyRequest {}); // 準備無參數請求內容
    let game_set: GameSet = client.new_game(request)    // 發送請求
        .await?.into_inner();                           // gRPC的GameSet物件
    Ok((
        game_set.id,
        game_set.game.unwrap().into()                   // 利用剛剛的From轉置
    ))
}

#[tauri::command]
pub async fn get_game_grpc(id: u32, ctx: State<'_, Context>) -> Result<CoreGame, ErrorResponse> {
    let mut client = TicTacToeClient::new(ctx.channel());
    let request = tonic::Request::new(IdRequest { id });
    Ok(client.get_game(request).await?.into_inner().into())
}

#[tauri::command]
pub async fn play_game_grpc(id: u32, num: u32, ctx: State<'_, Context>) -> Result<CoreGame, ErrorResponse> {
    let mut client = TicTacToeClient::new(ctx.channel());
    let request = tonic::Request::new(PlayRequest { id, num });
    Ok(client.play(request).await?.into_inner().into())
}

#[tauri::command]
pub async fn delete_game_grpc(id: u32, ctx: State<'_, Context>) -> Result<(), ErrorResponse> {
    let mut client = TicTacToeClient::new(ctx.channel());
    let request = tonic::Request::new(IdRequest { id });
    let _ = client.delete_game(request).await?;
    Ok(())
}

