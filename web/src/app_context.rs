use std::{sync::Arc, collections::HashMap};
use tokio::sync::{RwLock, mpsc::UnboundedSender};
use warp::ws::Message;

#[derive(Clone)]
pub struct AppContext {
    pub ws_connections:
    Arc<
        RwLock<
            HashMap<
                usize,
                UnboundedSender<Message>
            >
        >
    >,
}

impl Default for AppContext {
    fn default() -> Self {
        Self {
            ws_connections:
            Arc::new(
                RwLock::new(
                    HashMap::new()
                )
            ),
        }
    }
}