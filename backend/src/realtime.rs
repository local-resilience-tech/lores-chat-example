use axum::{
    extract::ws::{Message, WebSocket},
    extract::WebSocketUpgrade,
    response::Response,
};
use futures_util::StreamExt;

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut ws: WebSocket) {
    while let Some(Ok(msg)) = ws.next().await {
        if matches!(msg, Message::Close(_)) {
            return;
        }
        println!("Received message: {:?}", msg);
    }
}
