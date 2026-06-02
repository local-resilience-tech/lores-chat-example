use axum::{routing::get, Router};
use lores_p2panda_client::PandaClient;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use crate::static_server::frontend_handler;

mod api;
mod realtime;
mod static_server;

const PANDA_GRPC_ADDR: &str = "http://127.0.0.1:50051";
const APP_NAMESPACE: &str = "chat-example:v1";

#[derive(Clone)]
pub struct AppState {
    pub panda: Arc<Mutex<PandaClient>>,
    pub channels: Arc<Mutex<HashMap<[u8; 32], broadcast::Sender<Vec<u8>>>>>,
    pub app_namespace: String,
}

#[tokio::main]
async fn main() {
    let panda = PandaClient::connect_lazy(PANDA_GRPC_ADDR)
        .expect("failed to connect to panda gRPC endpoint");
    let panda = Arc::new(Mutex::new(panda));

    let state = AppState {
        panda,
        channels: Arc::new(Mutex::new(HashMap::new())),
        app_namespace: APP_NAMESPACE.to_string(),
    };

    let app = Router::new()
        .route("/test", get(|| async { "ok" }))
        .route("/api/regions", get(api::list_regions))
        .route("/ws/:region_id", get(realtime::handler))
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
