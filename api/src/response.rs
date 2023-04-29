use axum::response::{IntoResponse, Response};
use axum::Json;
use hyper::StatusCode;
use serde::Serialize;

pub struct ApiResponse<B: Serialize> {
    status: StatusCode,
    body: B,
}

impl<B: Serialize> ApiResponse<B> {
    pub fn ok(body: B) -> Self {
        ApiResponse {
            status: StatusCode::OK,
            body,
        }
    }

    pub fn created(body: B) -> Self {
        ApiResponse {
            status: StatusCode::CREATED,
            body,
        }
    }
}

impl<B: Serialize> IntoResponse for ApiResponse<B> {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
