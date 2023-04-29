use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use hyper::StatusCode;
use serde::Serialize;

use business::error::{BillError, InterestError};

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
    pub msg: String,
    pub timestamp: i64,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = ErrorResponse {
            msg,
            timestamp: Utc::now().timestamp(),
        };

        (status, Json(body)).into_response()
    }
}
