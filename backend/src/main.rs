use axum::{routing::get, Router};
use lores_p2panda_client::PandaClient;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::static_server::frontend_handler;

mod realtime;
mod static_server;

const PANDA_GRPC_ADDR_DEFAULT: &str = "http://127.0.0.1:50051";
const APP_ID: &str = "chat-example";
const INSTANCE_ID: &str = "instance1";

#[derive(Clone)]
pub struct AppState {
    pub panda: Arc<Mutex<PandaClient>>,
    pub channels: Arc<Mutex<HashMap<String, broadcast::Sender<Vec<u8>>>>>,
    pub app_id: String,
    pub instance_id: String,
}

#[tokio::main]
async fn main() {
    let panda_grpc_addr =
        std::env::var("PANDA_GRPC_ADDR").unwrap_or_else(|_| PANDA_GRPC_ADDR_DEFAULT.to_string());

    let panda = PandaClient::connect_lazy(panda_grpc_addr)
        .expect("failed to connect to panda gRPC endpoint");
    let panda = Arc::new(Mutex::new(panda));

    let state = AppState {
        panda,
        channels: Arc::new(Mutex::new(HashMap::new())),
        app_id: APP_ID.to_string(),
        instance_id: INSTANCE_ID.to_string(),
    };

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
