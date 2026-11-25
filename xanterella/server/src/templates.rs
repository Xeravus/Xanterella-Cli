use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use serde_json::{Value, json};
use xanterella_core::{
    database::db::DBTemplates,
    prolyxena::CreateTemplate,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState, NixValuesExtract};

#[derive(Serialize)]
pub struct ApiTemplatesResponse {
    pub len: i64,
    pub templates: Vec<TemplatesResponse>,
}

#[derive(Serialize)]
pub struct TemplatesResponse {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub config: String,
}

impl From<DBTemplates> for TemplatesResponse {
    fn from(db_template: DBTemplates) -> Self {
        Self {
            id: db_template.id,
            name: db_template.name,
            path: db_template.path,
            config: db_template.config.0.format_nix_values(),
        }
    }
}

pub async fn create_template(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateTemplate>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Template '{}'", payload.name),
    });
    state.db.add_template(&payload.name, &payload.path, payload.config).await.map_err(|_| ApiError::InternalError)?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Template '{}' added", payload.name),
    });

    Ok(Json(json!({
        "status": "success",
        "name": payload.name
    })))
}

pub async fn delete_template(
    State(state): State<Arc<AppState>>, Path(name): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Delete Template '{}'", name),
    });
    let deleted = state.db.delete_template(&name).await.map_err(|_| ApiError::InternalError)?;

    if deleted == 0 {
        return Err(ApiError::NotFound);
    }

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Template '{}' deleted", name),
    });
    Ok(Json(json!({
        "status": "success",
        "name": name,
    })))
}

pub async fn list_templates(State(state): State<Arc<AppState>>) -> Result<Json<ApiTemplatesResponse>, ApiError> {
    let templates = state.db.list_templates().await.map_err(|_| ApiError::InternalError)?;
    let len = templates.len();
    let response: Vec<TemplatesResponse> = templates.into_iter().map(TemplatesResponse::from).collect();
    Ok(Json(ApiTemplatesResponse {
        len: len.try_into().unwrap(),
        templates: response,
    }))
}

pub async fn check_template(
    State(state): State<Arc<AppState>>, Path(name): Path<String>,
) -> Result<Json<DBTemplates>, ApiError> {
    let template = state.db.get_template(&name).await.map_err(|_| ApiError::InternalError)?;
    match template {
        Some(t) => Ok(Json(t)),
        None => Err(ApiError::NotFound),
    }
}
