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
    handlers::api_handler::{get_buckets, home, protected}, 
    middleware::rate_limit::rate_limit, 
    state::app_state::AppState
};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/", get(home))
        .route("/protected", get(protected))
        .route_layer(from_fn_with_state(state.clone(), rate_limit))
        .route("/buckets", get(get_buckets))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Servidor rodando em http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}