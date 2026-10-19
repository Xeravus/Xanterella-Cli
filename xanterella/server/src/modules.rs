use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use indexmap::IndexMap;
use prolyxena::engine::{core::NixValue, formater::core::Format};
use serde_json::{Value, json};
use xanterella_core::{
    database::db::DBModul,
    prolyxena::CreateModul,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

#[derive(serde::Serialize)]
pub struct ModulesResponse {
    pub id: i64,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub options: String,
    pub config: String,
}

impl From<DBModul> for ModulesResponse {
    fn from(db_modul: DBModul) -> Self {
        let process_values = |vec: &Vec<serde_json::Value>| {
            vec.iter()
                .map(|val| {
                    if let Ok(nix_ast) = serde_json::from_value::<NixValue>(val.clone()) {
                        return nix_ast.format_nix(0);
                    }
                    if let Ok(map) = serde_json::from_value::<IndexMap<String, NixValue>>(val.clone()) {
                        return NixValue::AttrSet(map).format_nix(0);
                    }
                    format!("Deserialisierungs-Fehler beim Lesen. \n Ursprüngliches JSON: {} ", val)
                })
                .collect::<Vec<String>>()
                .join("\n")
        };

        Self {
            id: db_modul.id,
            name: db_modul.name,
            desc: db_modul.desc,
            category: db_modul.category,
            options: process_values(&db_modul.options.0),
            config: process_values(&db_modul.config.0),
        }
    }
}

pub async fn create_modul(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateModul>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Modul '{}'", payload.name),
    });
    state
        .db
        .add_modul(&payload.name, &payload.desc, &payload.category, payload.options, payload.config)
        .await
        .map_err(|_| ApiError::InternalError)?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Modul '{}' added", payload.name),
    });

    Ok(Json(json!({
        "status": "success",
        "modulname": payload.name
    })))
}

pub async fn delete_modul(
    State(state): State<Arc<AppState>>, Path(name): Path<String>,
) -> Result<Json<Value>, ApiError> {
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

pub async fn list_modules(State(state): State<Arc<AppState>>) -> Result<Json<Vec<ModulesResponse>>, ApiError> {
    let modules = state.db.list_modules().await.map_err(|_| ApiError::InternalError)?;
    let response: Vec<ModulesResponse> = modules.into_iter().map(ModulesResponse::from).collect();
    Ok(Json(response))
}

pub async fn check_modul(
    State(state): State<Arc<AppState>>, Path(name): Path<String>,
) -> Result<Json<DBModul>, ApiError> {
    let modul = state.db.get_modul(&name).await.map_err(|_| ApiError::InternalError)?;
    match modul {
        Some(h) => Ok(Json(h)),
        None => Err(ApiError::NotFound),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_helper::tests_utils::test_state;

    #[tokio::test]
    async fn test_api_modules_crud() {
        let state = test_state().await;
        let app_state = State(state.clone());

        let payload = Json(xanterella_core::prolyxena::CreateModul {
            name: "api-modul".to_string(),
            desc: "Test API Modul".to_string(),
            category: "api-tests".to_string(),
            options: vec![],
            config: vec![],
        });
        let res_create = create_modul(app_state.clone(), payload).await;
        assert!(res_create.is_ok());

        let list = list_modules(app_state.clone()).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "api-modul");

        let check = check_modul(app_state.clone(), Path("api-modul".to_string())).await.unwrap();
        assert_eq!(check.name, "api-modul");

        let res_del = delete_modul(app_state.clone(), Path("api-modul".to_string())).await;
        assert!(res_del.is_ok());
    }
}
