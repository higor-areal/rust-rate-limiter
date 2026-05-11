use axum::{
    Json,
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub status_code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub message: String,
}


pub fn handle_response(
    status: StatusCode,
    message: String,
) -> Result<Json<SuccessResponse>, Json<ErrorResponse>> {
    if (400..=599).contains(&status.as_u16()) {
        Err(Json(ErrorResponse {
            status_code: status.as_u16(),
            message: format!("Error: {}", message),
        }))
    } else {
        Ok(Json(SuccessResponse {
            status_code: status.as_u16(),
            message: message,
        }))
    }
}

