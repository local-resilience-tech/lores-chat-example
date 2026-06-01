use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::panda_client::PandaClient;
use crate::static_server::frontend_handler;

mod panda_client;
mod realtime;
mod static_server;

#[macro_use]
extern crate lazy_static;

const PANDA_GRPC_ADDR: &str = "http://127.0.0.1:50051";

#[derive(Clone)]
pub struct AppState {
    pub panda: Arc<Mutex<PandaClient>>,
    pub tx: broadcast::Sender<Vec<u8>>,
}

#[tokio::main]
async fn main() {
    let panda = PandaClient::new(PANDA_GRPC_ADDR).expect("invalid gRPC address");
    let panda = Arc::new(Mutex::new(panda));

    let (tx, _) = broadcast::channel(256);

    tokio::spawn(subscribe_loop(Arc::clone(&panda), tx.clone()));

    let state = AppState { panda, tx };

    let app = Router::new()
        .route("/test", get(|| async { "ok" }))
        .route("/ws", get(realtime::handler))
        .fallback_service(get(frontend_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    axum::serve(listener, app)
        .await
        .expect("backend server error");
}

async fn subscribe_loop(panda: Arc<Mutex<PandaClient>>, tx: broadcast::Sender<Vec<u8>>) {
    loop {
        let stream_result = {
            let mut client = panda.lock().await;
            client.subscribe().await
        };
        match stream_result {
            Ok(mut stream) => loop {
                match stream.message().await {
                    Ok(Some(event)) => {
                        // Ignore send errors — no active receivers is fine
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
