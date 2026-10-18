use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use indexmap::IndexMap;
use prolyxena::engine::{core::NixValue, formater::core::Format};
use serde_json::{Value, json};
use xanterella_core::{
    database::db::DBProfile,
    prolyxena::CreateProfile,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

#[derive(serde::Serialize)]
pub struct ProfileResponse {
    pub id: i64,
    pub name: String,
    pub dir: String,
    pub options: String,
}

impl From<DBProfile> for ProfileResponse {
    fn from(db_profile: DBProfile) -> Self {
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
            id: db_profile.id,
            name: db_profile.name,
            dir: db_profile.dir,
            options: process_values(&db_profile.options.0),
        }
    }
}

pub async fn create_profile(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateProfile>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Profile '{}'", payload.name),
    });
    state.db.add_profile(&payload.name, &payload.dir, payload.options).await.map_err(|_| ApiError::InternalError)?;

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

pub async fn list_profiles(State(state): State<Arc<AppState>>) -> Result<Json<Vec<ProfileResponse>>, ApiError> {
    let profiles = state.db.list_profiles().await.map_err(|_| ApiError::InternalError)?;
    let response: Vec<ProfileResponse> = profiles.into_iter().map(ProfileResponse::from).collect();
    Ok(Json(response))
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

pub async fn profile_add_option(
    State(state): State<Arc<AppState>>, Path(name): Path<String>, Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Option to Profile '{}'", name),
    });
    state.db.profile_add_option(&name, payload).await.map_err(|_| ApiError::InternalError)?;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Added Option to Profile '{}'", name),
    });
    Ok(Json(json!({
        "status": "success",
        "profile": name,
    })))
}

pub async fn profile_remove_option(
    State(state): State<Arc<AppState>>, Path(name): Path<String>, Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Remove Option from Profile '{}'", name),
    });
    state.db.profile_remove_option(&name, payload).await.map_err(|_| ApiError::InternalError)?;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Removed Option from Profile '{}'", name),
    });
    Ok(Json(json!({
        "status": "success",
        "profile": name,
    })))
}

#[cfg(test)]
mod tests {
    use tokio::sync::broadcast;
    use xanterella_core::database::db::Database;

    use super::*;

    async fn test_state() -> Arc<AppState> {
        let (tx, _rx) = broadcast::channel(100);
        let db = Database::init("sqlite::memory:").await.unwrap();
        Arc::new(AppState {
            tx,
            db,
        })
    }

    #[tokio::test]
    async fn test_api_profiles_crud() {
        let state = test_state().await;
        let app_state = State(state.clone());

        let payload = Json(xanterella_core::prolyxena::CreateProfile {
            name: "api-profile".to_string(),
            dir: "server".to_string(),
            options: vec![],
        });
        let res_create = create_profile(app_state.clone(), payload).await;
        assert!(res_create.is_ok());

        let list = list_profiles(app_state.clone()).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "api-profile");

        let check = check_profile(app_state.clone(), Path("api-profile".to_string())).await.unwrap();
        assert_eq!(check.name, "api-profile");

        let _ =
            profile_add_option(app_state.clone(), Path("api-profile".to_string()), Json(json!({"enable": true}))).await;
        let check_opt = check_profile(app_state.clone(), Path("api-profile".to_string())).await.unwrap();
        assert_eq!(check_opt.options.0.len(), 1);

        let _ =
            profile_remove_option(app_state.clone(), Path("api-profile".to_string()), Json(json!({"enable": true})))
                .await;
        let check_opt_rm = check_profile(app_state.clone(), Path("api-profile".to_string())).await.unwrap();
        assert!(check_opt_rm.options.0.is_empty());

        let res_del = delete_profile(app_state.clone(), Path("api-profile".to_string())).await;
        assert!(res_del.is_ok());
    }
}
