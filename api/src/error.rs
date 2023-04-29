use axum::response::{IntoResponse, Response};
use axum::Json;
use business::error::{BillError, InterestError};
use hyper::StatusCode;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    NotFound(String),
    BadRequest(String),
}

impl From<BillError> for ApiError {
    fn from(bill_error: BillError) -> Self {
        match bill_error {
            BillError::NotFound(msg) => ApiError::NotFound(msg),
            BillError::Persistence(msg) => ApiError::InternalServerError(msg),
            BillError::ParseError(msg) => ApiError::BadRequest(msg),
        }
    }
}

impl From<InterestError> for ApiError {
    fn from(interest_error: InterestError) -> Self {
        match interest_error {
            InterestError::NotFound(msg) => ApiError::NotFound(msg),
            InterestError::Persistence(msg) => ApiError::InternalServerError(msg),
            InterestError::ParseError(msg) => ApiError::BadRequest(msg),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
        };

        let body = ErrorResponse { message };

        (status, Json(body)).into_response()
    }
}
