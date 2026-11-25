pub mod app;
pub mod config;
pub mod deploy;
pub mod generate;
pub mod hosts;
pub mod modules;
pub mod profiles;
pub mod raw;
pub mod templates;
pub mod test_helper;

use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};
use indexmap::IndexMap;
use prolyxena::engine::{core::NixValue, formater::core::Format};
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
    pub paths: AppPaths,
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

pub(crate) trait NixValuesExtract {
    fn format_nix_values(&self) -> String;
}

impl NixValuesExtract for [serde_json::Value] {
    fn format_nix_values(&self) -> String {
        self.iter()
            .map(|val| {
                if let Ok(nix_ast) = serde_json::from_value::<NixValue>(val.clone()) {
                    return nix_ast.format_nix(0);
                }
                if let Ok(map) = serde_json::from_value::<IndexMap<String, NixValue>>(val.clone()) {
                    return NixValue::AttrSet(map).format_nix(0);
                }
                format!("Deserialisierungs-Fehler beim Lesen. \n Ursprüngliches JSON: {}", val)
            })
            .collect::<Vec<String>>()
            .join("\n")
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
        paths: paths.clone(),
    });

    let app = create_app(state);

    let ip = format!("{}:{}", config.ip, config.port);
    let listener = tokio::net::TcpListener::bind(&ip).await.expect("Failed to bind Tcp Listener");
    println!("Server running on http://{}", ip);

    axum::serve(listener, app).await.expect("Failed to start Server");
}

#[cfg(test)]
#[path = "../tests/support/router.rs"]
mod router_integration;
