use crate::ApiError;

use serde_json::{Value, json};
use axum::{Json, Router, extract::Path, response::IntoResponse, routing::get};

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user))
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
