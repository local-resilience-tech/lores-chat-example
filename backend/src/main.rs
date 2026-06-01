use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::panda_client::PandaClient;
use crate::static_server::frontend_handler;

mod panda_client;
mod realtime;
mod static_server;

#[macro_use]
extern crate lazy_static;

const PANDA_GRPC_ADDR: &str = "http://127.0.0.1:50051";

#[tokio::main]
async fn main() {
    let panda = PandaClient::new(PANDA_GRPC_ADDR).expect("invalid gRPC address");
    let panda = Arc::new(Mutex::new(panda));

    let app = Router::new()
        .route("/test", get(|| async { "ok" }))
        .route("/ws", get(realtime::handler))
        .fallback_service(get(frontend_handler))
        .with_state(panda);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    axum::serve(listener, app)
        .await
        .expect("backend server error");
}
