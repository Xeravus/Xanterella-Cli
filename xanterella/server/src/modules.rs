use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use xanterella_core::{xanterella::{EventFormat, EventState}, db::DBModul};

use crate::{ApiError, AppState};

#[derive(Serialize, Deserialize)]
pub struct CreateModul {
    pub name: String,
    pub desc: String,
    pub category: String,
    pub options: Vec<Value>,
}

pub async fn create_modul(State(state): State<Arc<AppState>>, Json(payload): Json<CreateModul>) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Modul '{}'", payload.name),
    });
    state.db.add_modul(&payload.name, &payload.desc, &payload.category, payload.options).await.map_err(|_| ApiError::InternalError )?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Modul '{}' added", payload.name),
    });

    Ok(Json(json!({
        "status": "success",
        "modulname": payload.name
    })))
}

pub async fn delete_modul(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Delete Modul '{}'", name),
    });
    let deleted = state.db.delete_modul(&name).await.map_err(|_| ApiError::InternalError)?;

    if deleted == 0 {
        return Err(ApiError::NotFound);
    }

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Modul '{}' deleted", name),
    });
    Ok(Json(json!({
        "status": "success",
        "modulname": name 
    })))
}

pub async fn list_modules(State(state): State<Arc<AppState>>) -> Result<Json<Vec<DBModul>>, ApiError> {
    let modules = state.db.list_modules().await.map_err(|_| ApiError::InternalError)?;
    Ok(Json(modules))
}

pub async fn check_modul(State(state): State<Arc<AppState>>, Path(name): Path<String>) -> Result<Json<DBModul>, ApiError> {
    let modul = state.db.get_modul(&name).await.map_err(|_| ApiError::InternalError)?;
    match modul {
        Some(h) => Ok(Json(h)),
        None => Err(ApiError::NotFound),
    }
}
