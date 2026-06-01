use axum::{
    extract::ws::{Message, WebSocket},
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use futures_util::StreamExt;

use crate::AppState;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut ws: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();
    loop {
        tokio::select! {
            ws_msg = ws.next() => {
                let msg = match ws_msg {
                    Some(Ok(m)) => m,
                    _ => return,
                };
                if matches!(msg, Message::Close(_)) {
                    return;
                }
                let payload = match msg {
                    Message::Text(text) => text.as_bytes().to_vec(),
                    Message::Binary(bytes) => bytes.to_vec(),
                    _ => continue,
                };
                let mut client = state.panda.lock().await;
                if let Err(e) = client.publish(payload).await {
                    eprintln!("Failed to publish message: {e}");
                    let error_msg = format!(
                        r#"{{"type":"error","message":{}}}"#,
                        serde_json::json!(e.to_string())
                    );
                    let _ = ws.send(Message::Text(error_msg.into())).await;
                }
            }
            Ok(payload) = rx.recv() => {
                if let Ok(text) = String::from_utf8(payload) {
                    let _ = ws.send(Message::Text(text.into())).await;
                }
            }
        }
    }
}
