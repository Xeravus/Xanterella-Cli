use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use serde_json::{Value, json};
use xanterella_core::{
    database::db::DBRaw,
    prolyxena::CreateRaw,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

#[derive(Serialize)]
pub struct ApiRawResponse {
    pub len: i64,
    pub raw: Vec<RawResponse>,
}

#[derive(Serialize)]
pub struct RawResponse {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub content: String,
    pub readable: bool,
}

impl From<DBRaw> for RawResponse {
    fn from(db_raw: DBRaw) -> Self {
        Self {
            id: db_raw.id,
            name: db_raw.name,
            path: db_raw.path,
            content: if db_raw.readable {
                String::from_utf8_lossy(&db_raw.content).into_owned()
            } else {
                " --- Cryptic Text --- ".to_string()
            },
            readable: db_raw.readable,
        }
    }
}

pub async fn create_raw(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateRaw>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Raw '{}'", payload.name),
    });
    state
        .db
        .add_raw(&payload.name, &payload.path, payload.content, payload.readable)
        .await
        .map_err(|_| ApiError::InternalError)?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Raw '{}' added", payload.name),
    });

    Ok(Json(json!({
        "status": "success",
        "name": payload.name
    })))
}

pub async fn delete_raw(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Delete Raw '{}'", name),
    });
    let deleted = state.db.delete_raw(&name).await.map_err(|_| ApiError::InternalError)?;

    if deleted == 0 {
        return Err(ApiError::NotFound);
    }

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Raw '{}' deleted", name),
    });
    Ok(Json(json!({
        "status": "success",
        "name": name,
    })))
}

pub async fn list_raw(State(state): State<Arc<AppState>>) -> Result<Json<ApiRawResponse>, ApiError> {
    let raw = state.db.list_raw().await.map_err(|_| ApiError::InternalError)?;
    let len = raw.len();
    let response: Vec<RawResponse> = raw.into_iter().map(RawResponse::from).collect();
    Ok(Json(ApiRawResponse {
        len: len.try_into().unwrap(),
        raw: response,
    }))
}

pub async fn check_raw(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Result<Json<DBRaw>, ApiError> {
    let raw = state.db.get_raw(&name).await.map_err(|_| ApiError::InternalError)?;
    match raw {
        Some(t) => Ok(Json(t)),
        None => Err(ApiError::NotFound),
    }
}
