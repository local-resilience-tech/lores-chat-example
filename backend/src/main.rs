use axum::{routing::get, Router};
use std::net::SocketAddr;

use crate::static_server::frontend_handler;

mod realtime;
mod static_server;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/test", get(|| async { "ok" }))
        .route("/ws", get(realtime::handler))
        .fallback_service(get(frontend_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend listener");

    axum::serve(listener, app)
        .await
        .expect("backend server error");
}
