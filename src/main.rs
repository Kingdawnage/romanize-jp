use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tracing_subscriber::filter::LevelFilter;

use crate::{error::HttpError, models::RomanizeRequest};

mod error;
mod models;
mod romanize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let port = 8080;

    let app: Router = Router::new()
        .route("/", get(|| async { Json("Hello, World!") }))
        .route("/romanize", post(romanize))
        .route("/health", get(health_check));

    println!("Server running on http://localhost:{}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn romanize(Json(payload): Json<RomanizeRequest>) -> Result<impl IntoResponse, HttpError> {
    if payload.text.is_empty() {
        return Err(HttpError::bad_request("Text cannot be empty"));
    }

    let romanized_text = romanize::romanize_text(&payload.text)
        .map_err(|e| HttpError::server_error(format!("Romanization failed: {}", e)))?;

    Ok(Json(models::RomanizeResponse {
        original_text: payload.text.clone(),
        romanized_text,
    }))
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "message": "Service is running"
    }))
}
