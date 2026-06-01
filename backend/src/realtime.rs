use axum::{
    extract::ws::{Message, WebSocket},
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::panda_client::PandaClient;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(panda): State<Arc<Mutex<PandaClient>>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, panda))
}

async fn handle_socket(mut ws: WebSocket, panda: Arc<Mutex<PandaClient>>) {
    while let Some(Ok(msg)) = ws.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }
        println!("Received message: {:?}", msg);

        let payload = match &msg {
            Message::Text(text) => text.as_bytes().to_vec(),
            Message::Binary(bytes) => bytes.to_vec(),
            _ => continue,
        };

        let mut client = panda.lock().await;
        if let Err(e) = client.publish(payload).await {
            eprintln!("Failed to publish message: {e}");
        }
    }
}
