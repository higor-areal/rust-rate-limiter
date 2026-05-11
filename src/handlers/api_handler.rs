use std::{
    collections::HashMap, 
    sync::Arc
};

use serde_json::{Value, json};
use tokio::sync::Mutex;

use axum::{
    Json, extract::State, handler, http::{HeaderMap, header, StatusCode}, response::{Response, IntoResponse}
};

use crate::{
    limiter::token_bucket::Bucket, 
    state::{app_state::AppState},
    responses::response::{SuccessResponse, ErrorResponse, handle_response},
    middleware::rate_limit::get_token
};

pub async fn home() -> Json<Value>{
    Json(json!({
  "message": "Rust Rate Limiter API"
}))
}

pub async fn protected() -> Result<Json<SuccessResponse>, Json<ErrorResponse>>{
    handle_response (  201, "request allowed".to_string() ) 
}

pub async fn get_buckets(State(state): State<Arc<Mutex<AppState>>>) -> Json<HashMap<String, Bucket>> {
    let data = state.lock().await;

    Json(data.buckets.clone())
}

pub async fn stats(
    State(state): State<Arc<Mutex<AppState>>>,
    header: HeaderMap,
) -> Response {
    let token = match get_token(&header) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "Unauthorized: Missing token",
            )
                .into_response();
        }
    };

    let mut data = state.lock().await;

    let bucket = data
        .buckets
        .entry(token)
        .or_insert_with(|| Bucket::new(10.0, 0.5));

    Json(bucket.clone()).into_response()
}