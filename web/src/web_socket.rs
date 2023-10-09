use std::sync::atomic::{AtomicUsize, Ordering};
use futures_util::{SinkExt, TryFutureExt, stream::StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Filter, Rejection, Reply, ws::{Message, WebSocket}};
use crate::app_context::AppContext;

static NEXT_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);  // 遞增取號用

pub fn ws_routers(context: AppContext)
                  -> impl Filter<Extract=impl Reply, Error=Rejection> + Clone {
    warp::path("echo")
        .and(warp::ws())
        .and(warp::any().map(move || context.clone()))
        .map(|ws: warp::ws::Ws, ctx: AppContext| {            // 注入State
            ws.on_upgrade(move |socket| ws_connected(socket, ctx))
        })                   // ↑↑ 接到連線使用我們定義的ws_connected fn處理
}

async fn ws_connected(ws: WebSocket, ctx: AppContext) {
    let conn_id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);  // 取號
    tracing::info!("new websocket connection: {}", conn_id);

    let (mut ws_tx, mut ws_rx) = ws.split();       // 取得 web_socket 的 tx/rx
    let (tx, rx) = mpsc::unbounded_channel();      // 建立一個 mpsc 通道
    let mut rx = UnboundedReceiverStream::new(rx); // mpsc 收受端

    // 等待程式通知要發送訊息時的處理，透過mpsc接受訊息
    tokio::task::spawn(async move {                 // 建一獨立工作(分身)監聽
        while let Some(message) = rx.next().await { // 當 mpsc 收到訊息時
            ws_tx
                .send(message)                      // 從 web_socket 發送該訊息
                .unwrap_or_else(|e| {
                    tracing::info!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // 把連線id和 mpsc 的發送端，存到AppState裡，後續我們在程式任意地方都可以呼叫這個tx
    let _ = &ctx.ws_connections.write().await.insert(conn_id, tx);

    // 處理當 websocket 接收到資料的時候
    while let Some(result) = ws_rx.next().await {
        let msg = match result {  // 解析收到的訊息
            Ok(msg) => msg,
            Err(e) => {
                tracing::info!("websocket error(uid={}): {}", conn_id, e);
                break;
            }
        };
        // 處理訊息，本例是發送給websocket全連線端
        send_all_message(conn_id, msg, &ctx).await;
    }

    // 處理斷線處理
    disconnected(conn_id, &ctx).await;
}

/// 處理訊息發送給指定連線id用戶端
#[allow(dead_code)]
async fn send_one_message(my_id: usize, msg: Message, ctx: &AppContext) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);  // 訊息帶用戶id

    // 找出特定 id 的 tx，並傳送文字訊息
    for (_, tx) in ctx.ws_connections.read().await.iter().filter(|(&uid, _)| uid == my_id) {
        if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {}
    }
}

/// 處理訊息發送給所有連線用戶端
async fn send_all_message(my_id: usize, msg: Message, ctx: &AppContext) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);  // 訊息帶用戶id

    // 遍歷所有的連線端，傳送文字訊息
    for (_, tx) in ctx.ws_connections.read().await.iter() {
        if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {}
    }
}

/// 處理websocket連線中斷
async fn disconnected(conn_id: usize, ctx: &AppContext) {
    tracing::info!("disconnected conn_id: {}", conn_id);
    ctx.ws_connections.write().await.remove(&conn_id);   // 從AppState移除該連線tx通道
}

use tokio::{time::Duration, time};
use rand::random;

pub async fn polling_message(ctx: &AppContext) {
    let ctx = ctx.clone();
    // 因為要一直存活才能處理，故開分身：
    tokio::task::spawn(async move {
        loop {
            let secs = random::<u64>() % 9_000 + 1_000; // 產生隨機等待豪秒
            time::sleep(Duration::from_millis(secs)).await;  // 等待

            // 從訊息庫隨機取訊息並發送至用戶端
            let message = my_core::game_message::message_factory();
            send_all_message(0, Message::text(message), &ctx).await;
        }
    });
}