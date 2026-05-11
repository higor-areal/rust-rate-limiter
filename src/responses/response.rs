use axum::{
    Json,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub status_code: u32,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status_code: u32,
    pub message: String,
}


pub fn handle_response(
    status: u32,
    message: String,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    if (400..=599).contains(&status) {
        Err(Json(ErrorResponse {
            status_code: status,
            message: format!("Error: {}", message),
        }))
    } else {
        Ok(Json(SuccessResponse {
            status_code: status,
            message: message,
        }))
    }
}

