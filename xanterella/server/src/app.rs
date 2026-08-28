use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use serde_json::{Value, json};
use xanterella_core::{Ping, Xanterella, XanterellaInstall};

use crate::ApiError;
use crate::AppState;

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new().route("/health", get(health_check)).route("/ping/:ip", get(get_ping)).with_state(state)
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}

pub async fn get_ping(State(state): State<Arc<AppState>>, Path(ip): Path<String>) -> Result<Json<Value>, ApiError> {
    let xanterella = Xanterella::new();
    let mut install = XanterellaInstall::new(xanterella);
    install.xanterella.set_sender(state.tx.clone());
    install.set_ip(&ip);

    let result = tokio::task::spawn_blocking(move || install.ping()).await.map_err(|_| ApiError::InternalError)?;

    match result {
        Ok(_) => Ok(Json(json!({
            "status": "ok",
            "ip": ip,
        }))),
        Err(_) => Err(ApiError::InternalError),
    }
}
