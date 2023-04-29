use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

pub struct ApiResponse<Body: Serialize> {
    status: StatusCode,
    body: Body,
}

impl<Body: Serialize> ApiResponse<Body> {
    pub fn ok(body: Body) -> Self {
        ApiResponse {
            status: StatusCode::OK,
            body,
        }
    }

    pub fn created(body: Body) -> Self {
        ApiResponse {
            status: StatusCode::CREATED,
            body,
        }
    }
}

impl<Body: Serialize> IntoResponse for ApiResponse<Body> {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
