use std::{
    collections::HashMap, 
    sync::Arc
};
use tokio::sync::Mutex;

use axum::{
    extract::State,
    Json
};

use crate::{
    limiter::token_bucket::Bucket, 
    state::{app_state::AppState}
};

pub async fn home() -> String{
    "Home app".to_string()
}

pub async fn get_buckets(State(state): State<Arc<Mutex<AppState>>>) -> Json<HashMap<String, Bucket>> {
    let data = state.lock().await;

    Json(data.buckets.clone())
}