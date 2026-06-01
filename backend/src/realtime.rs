use axum::{
    extract::ws::{Message, WebSocket},
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::{panda_client::PandaClient, AppState};

pub async fn handler(
    Path(region_id_hex): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    let region_id = match parse_hex_32(&region_id_hex) {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "invalid region_id").into_response(),
    };
    ws.on_upgrade(move |socket| handle_socket(socket, state, region_id))
}

async fn handle_socket(mut ws: WebSocket, state: AppState, region_id: [u8; 32]) {
    let mut rx = get_or_create_channel(&state, region_id).await;

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
                if let Err(e) = client.publish(region_id, &state.app_namespace, payload).await {
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
    region_id: [u8; 32],
) -> broadcast::Receiver<Vec<u8>> {
    let mut channels = state.channels.lock().await;
    if let Some(tx) = channels.get(&region_id) {
        return tx.subscribe();
    }
    let (tx, rx) = broadcast::channel(256);
    channels.insert(region_id, tx.clone());
    // Drop the lock before spawning so the loop can acquire it if needed.
    drop(channels);
    tokio::spawn(subscribe_loop(
        Arc::clone(&state.panda),
        tx,
        region_id,
        state.app_namespace.clone(),
    ));
    rx
}

pub async fn subscribe_loop(
    panda: Arc<Mutex<PandaClient>>,
    tx: broadcast::Sender<Vec<u8>>,
    region_id: [u8; 32],
    app_namespace: String,
) {
    loop {
        let stream_result = {
            let mut client = panda.lock().await;
            client.subscribe(region_id, &app_namespace).await
        };
        match stream_result {
            Ok(mut stream) => loop {
                match stream.message().await {
                    Ok(Some(event)) => {
                        // Ignore send errors — no active receivers is fine.
                        let _ = tx.send(event.payload);
                    }
                    Ok(None) => break, // server closed stream, reconnect
                    Err(e) => {
                        eprintln!("Subscribe stream error: {e}");
                        break;
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to subscribe: {e}");
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

fn parse_hex_32(hex: &str) -> Option<[u8; 32]> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let hi = (chunk[0] as char).to_digit(16)?;
        let lo = (chunk[1] as char).to_digit(16)?;
        bytes[i] = (hi * 16 + lo) as u8;
    }
    Some(bytes)
}
