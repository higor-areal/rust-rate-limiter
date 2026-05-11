
use crate::{
    limiter::token_bucket::Bucket,
    state::app_state::AppState,
};
use axum::{
    extract::{Request, State}, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn rate_limit(
    State(state): State<Arc<Mutex<AppState>>>,
    request: Request,
    next: Next,
) -> Response {
    // Extrai o token (suporta "Bearer xxx" e "xxx")
    let token = match get_token(request.headers()) {
        Some(t) => t,
        None => {
            return (StatusCode::UNAUTHORIZED, "Unauthorized: Missing token").into_response();
        }
    };

    // Usa lock sincronizado (Mutex do std, não o do tokio)
    let mut data = state.lock().await;

    let bucket = data
        .buckets
        .entry(token)
        .or_insert_with(|| Bucket::new(10.0, 0.5));

    let allowed = bucket.try_consume();

    drop(data); // libera o lock o mais cedo possível

    if allowed {
        next.run(request).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "Too Many Requests")
            .into_response()
    }
}


pub fn get_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers
        .get("authorization")?
        .to_str()
        .ok()?
        .trim();

    // Remove "Bearer " ou "bearer " se existir
    let token = auth_header
        .strip_prefix("Bearer ")
        .or_else(|| auth_header.strip_prefix("bearer "))
        .unwrap_or(auth_header)   // se não tiver prefixo, usa a string original
        .trim()
        .to_string();

    // Validação mínima de segurança
    if token.is_empty() || token.len() < 3 {
        None
    } else {
        Some(token)
    }
}