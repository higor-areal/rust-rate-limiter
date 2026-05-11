use std::{
    collections::HashMap, 
    sync::Arc
};

use serde_json::{Value, json};
use tokio::sync::Mutex;

use axum::{
    Json, extract::State, http::{HeaderMap, StatusCode},
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
    handle_response (  StatusCode::OK, "request allowed".to_string() ) 
}

pub async fn get_buckets(State(state): State<Arc<Mutex<AppState>>>) -> Json<HashMap<String, Bucket>> {
    let data = state.lock().await;

    Json(data.buckets.clone())
}

pub async fn stats(
    State(state): State<Arc<Mutex<AppState>>>,
    header: HeaderMap,
) -> Result<Json<Bucket>, Json<ErrorResponse>> {
    let token = match get_token(&header) {
        Some(t) => t,
        None => {
            return Err(Json(ErrorResponse {
                status_code: 400,
                message: "Missing token".to_string(),
            }));
        }
    };

    let data = state.lock().await;

    if let Some(value) = data.buckets.get(&token) {
        Ok(Json(value.clone()))
    } else {
        Err(Json(ErrorResponse {
            status_code: 400,
            message: "Bucket not found".to_string(),
        }))
    }
}

pub async fn reset(
    State(state): State<Arc<Mutex<AppState>>>,
    header: HeaderMap,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    let token = match get_token(&header) {
        Some(t) => t,
        None => {
            return handle_response(StatusCode::UNAUTHORIZED, "Missing token".to_string())
        }
    };

    let mut data = state.lock().await;

    if let Some(_value) = data.buckets.remove_entry(&token){
        handle_response(StatusCode::OK, "bucket reset".to_string())
    } else {
        handle_response(StatusCode::BAD_REQUEST, "Missing token".to_string())
    }
    
}