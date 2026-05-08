mod handlers;
mod limiter;
mod middleware;
mod models;
mod responses;
mod state;

use axum::{
    Router,
    routing::get,
    middleware::from_fn_with_state,
};

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    state::app_state::AppState,
    handlers::api_handler::{home, get_buckets},
    middleware::rate_limit::rate_limit,
};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/", get(home))
        .route("/buckets", get(get_buckets))
        .route_layer(from_fn_with_state(state.clone(), rate_limit))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Servidor rodando em http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}