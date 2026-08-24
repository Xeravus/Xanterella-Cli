use axum::{Json, Router, extract::Path, response::IntoResponse, routing::get};
use serde_json::{Value, json};
use xanterella_core::{Ping, Xanterella, XanterellaInstall};

use crate::ApiError;

pub fn create_app() -> Router {
    Router::new().route("/health", get(health_check)).route("/ping/:ip", get(get_ping))
use crate::ApiError;

use serde_json::{Value, json};
use axum::{Json, Router, extract::Path, response::IntoResponse, routing::get};
use xanterella_core::{Ping, Xanterella, XanterellaInstall};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ping/:ip", get(get_ping))
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}

pub async fn list_users() -> Result<Json<Value>, ApiError> {
    Err(ApiError::InternalError)
}

pub async fn get_user(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
    if id > 100 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(json!({
        "id": id,
        "name": "User"
    })))
}

pub async fn get_ping(Path(ip): Path<String>) -> Result<Json<Value>, ApiError> {
    let mut xanterella = Xanterella::new();
    let mut install = XanterellaInstall::new(&mut xanterella);
    install.set_ip(&ip);

    let result = install.ping();

    match result {
        Ok(_) => Ok(Json(json!({
            "status": "ok",
            "ip": ip,
        }))),
        Ok(_) => {
            Ok(Json(json!({
                "status": "ok",
                "ip": ip,
            })))
        },
        Err(_) => Err(ApiError::InternalError),
    }
}
