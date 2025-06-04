use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError::new(message, StatusCode::BAD_REQUEST)
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
