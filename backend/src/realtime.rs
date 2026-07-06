use axum::{
    extract::ws::{Message, WebSocket},
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use futures_util::StreamExt;
use lores_p2panda_client::PandaClient;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::AppState;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut ws: WebSocket, state: AppState) {
    let instance_id = state.instance_id.clone();
    let mut rx = get_or_create_channel(&state, &instance_id).await;

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
                if let Err(e) = client.publish(&state.app_id, &instance_id, payload, None).await {
                    eprintln!("Failed to publish message: {e}");
                    let error_msg = format!(
                        r#"{{"type":"error","message":{}}}"#,
                        serde_json::json!(e.to_string())
                    );
                    let _ = ws.send(Message::Text(error_msg.into())).await;
                }
            }
            msg = rx.recv() => {
                match msg {
                    Ok(payload) => {
                        if let Ok(text) = String::from_utf8(payload) {
                            let _ = ws.send(Message::Text(text.into())).await;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => return,
                }
            }
        }
    }
}

async fn get_or_create_channel(
    state: &AppState,
    instance_id: &str,
) -> broadcast::Receiver<Vec<u8>> {
    let mut channels = state.channels.lock().await;
    if let Some(tx) = channels.get(instance_id) {
        return tx.subscribe();
    }
    let (tx, rx) = broadcast::channel(256);
    channels.insert(instance_id.to_string(), tx.clone());
    // Drop the lock before spawning so the loop can acquire it if needed.
    drop(channels);
    tokio::spawn(subscribe_loop(
        Arc::clone(&state.panda),
        tx,
        instance_id.to_string(),
        state.app_id.clone(),
    ));
    rx
}

pub async fn subscribe_loop(
    panda: Arc<Mutex<PandaClient>>,
    tx: broadcast::Sender<Vec<u8>>,
    instance_id: String,
    app_id: String,
) {
    loop {
        let stream_result = {
            let mut client = panda.lock().await;
            client.subscribe(&app_id, &instance_id).await
        };
        match stream_result {
            Ok(response) => {
                let mut stream = response.into_inner();
                let ok_msg = serde_json::json!({ "type": "subscribe_ok" })
                    .to_string()
                    .into_bytes();
                let _ = tx.send(ok_msg);
                loop {
                    match stream.message().await {
                        Ok(Some(event)) => {
                            // Ignore send errors — no active receivers is fine.
                            let author_node = hex::encode(&event.author);
                            let payload_text = String::from_utf8_lossy(&event.payload);
                            let envelope = serde_json::json!({
                                "author_node": author_node,
                                "text": payload_text,
                            })
                            .to_string()
                            .into_bytes();
                            let _ = tx.send(envelope);
                        }
                        Ok(None) => break, // server closed stream, reconnect
                        Err(e) => {
                            eprintln!("Subscribe stream error: {e}");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to subscribe: {e}");
                let error_msg = serde_json::json!({
                    "type": "subscribe_error",
                    "message": e.to_string(),
                })
                .to_string()
                .into_bytes();
                let _ = tx.send(error_msg);
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
