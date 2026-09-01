use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use xanterella_core::{
    db::DBProfile,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateProfile {
    pub name: String,
    pub dir: String,
    pub options: Vec<Value>,
}

pub async fn create_profile(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateProfile>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Profile '{}'", payload.name),
    });
    state
        .db
        .add_profile(&payload.name, &payload.dir, payload.options)
        .await
        .map_err(|_| ApiError::InternalError)?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Profile '{}' added", payload.name),
    });

    Ok(Json(json!({
        "status": "success",
        "profilename": payload.name
    })))
}

pub async fn delete_profile(
    State(state): State<Arc<AppState>>, Path(name): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Delete Profile '{}'", name),
    });
    let deleted = state.db.delete_profile(&name).await.map_err(|_| ApiError::InternalError)?;

    if deleted == 0 {
        return Err(ApiError::NotFound);
    }

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Profile '{}' deleted", name),
    });
    Ok(Json(json!({
        "status": "success",
        "profilename": name
    })))
}

pub async fn list_profiles(State(state): State<Arc<AppState>>) -> Result<Json<Vec<DBProfile>>, ApiError> {
    let profiles = state.db.list_profiles().await.map_err(|_| ApiError::InternalError)?;
    Ok(Json(profiles))
}

pub async fn check_profile(
    State(state): State<Arc<AppState>>, Path(name): Path<String>,
) -> Result<Json<DBProfile>, ApiError> {
    let modul = state.db.get_profile(&name).await.map_err(|_| ApiError::InternalError)?;
    match modul {
        Some(h) => Ok(Json(h)),
        None => Err(ApiError::NotFound),
    }
}
