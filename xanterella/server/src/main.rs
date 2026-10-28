pub mod app;
pub mod generate;
pub mod hosts;
pub mod modules;
pub mod profiles;
pub mod templates;
pub mod test_helper;

use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use tokio::sync::broadcast;
use xanterella_core::{
    config::{AppConfig, AppPaths},
    database::db::Database,
    xanterella::EventFormat,
};

use crate::app::*;

pub struct AppState {
    pub tx: broadcast::Sender<EventFormat>,
    pub db: Database,
    pub config: AppConfig,
}

#[allow(unused)]
#[derive(Debug)]
pub enum ApiError {
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
    let paths = AppPaths::init();
    let config = AppConfig::load(&paths);
    let db =
        Database::init(&paths.database_url).await.expect("Init-Fehler: Datenkbank konnte nicht initialisiert werden");

    let state = Arc::new(AppState {
        tx,
        db,
        config: config.clone(),
    });

    let app = create_app(state);

    let ip = format!("{}:{}", config.ip, config.port);
    let listener = tokio::net::TcpListener::bind(&ip).await.expect("Failed to bind Tcp Listener");
    println!("Server running on http://{}", ip);

    axum::serve(listener, app).await.expect("Failed to start Server");
}
