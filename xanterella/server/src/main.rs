use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tokio::sync::broadcast;
use xanterella_core::{xanterella::EventFormat, db::Database};

mod app;
use crate::app::*;

pub struct AppState {
    pub tx: broadcast::Sender<EventFormat>,
    pub db: Database
}

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
    let (tx, _rx) = broadcast::channel::<EventFormat>(100);
    let db = Database::init("sqlite://../../xanterella.db?mode=rwc")
        .await
        .expect("Datenkbank konnte nicht initialisiert werden");

    let state = Arc::new(AppState {
        tx,
        db, 
    });

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind Tcp Listener");
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("Failed to start Server");
}
