use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

mod app;

use crate::app::*;

#[allow(unused)]
#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            ApiError::InvalidInput(err) => (StatusCode::BAD_REQUEST, err),
            ApiError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };

        let body = Json(json!({
            "error": error_msg,
        }));
        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind Tcp Listener");
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("Failed to start Server");
}
