use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use serde_json::{Value, json};
use xanterella_core::{xanterella::{EventFormat, EventState}, db::DBHost};

use crate::{ApiError, AppState};

#[derive(Deserialize)]
pub struct CreateHost {
    pub hostname: String,
    pub ip: String,
}

pub async fn create_host(State(state): State<Arc<AppState>>, Json(payload): Json<CreateHost>) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Host '{}'", payload.hostname),
    });
    state.db.add_host(&payload.hostname, &payload.ip).await.map_err(|_| ApiError::InternalError )?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Host '{}' added", payload.hostname),
    });

    Ok(Json(json!({
        "status": "success",
        "hostname": payload.hostname
    })))
}

pub async fn delete_host(State(state): State<Arc<AppState>>, Path(hostname): Path<String>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Delete Host '{}'", hostname),
    });
    let deleted = state.db.delete_host(&hostname).await.map_err(|_| ApiError::InternalError)?;

    if deleted == 0 {
        return Err(ApiError::NotFound);
    }

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Host '{}' deleted", hostname),
    });
    Ok(Json(json!({
        "status": "success",
        "hostname": hostname 
    })))
}

pub async fn list_hosts(State(state): State<Arc<AppState>>) -> Result<Json<Vec<DBHost>>, ApiError> {
    let hosts = state.db.list_hosts().await.map_err(|_| ApiError::InternalError)?;
    Ok(Json(hosts))
}

pub async fn check_host(State(state): State<Arc<AppState>>, Path(hostname): Path<String>) -> Result<Json<DBHost>, ApiError> {
    let host = state.db.get_host(&hostname).await.map_err(|_| ApiError::InternalError)?;
    match host {
        Some(h) => Ok(Json(h)),
        None => Err(ApiError::NotFound),
    }
}
