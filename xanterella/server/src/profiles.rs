use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use xanterella_core::prolyxena::CreateProfile;
use xanterella_core::{
    database::db::DBProfile,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

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

        // 1. Create Profile
        let payload = Json(xanterella_core::prolyxena::CreateProfile {
            name: "api-profile".to_string(),
            dir: "server".to_string(),
            options: vec![],
        });
        let res_create = create_profile(app_state.clone(), payload).await;
        assert!(res_create.is_ok());

        // 2. List Profiles
        let list = list_profiles(app_state.clone()).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "api-profile");

        // 3. Check Profile
        let check = check_profile(app_state.clone(), Path("api-profile".to_string())).await.unwrap();
        assert_eq!(check.name, "api-profile");

        // 4. Add Option
        let _ =
            profile_add_option(app_state.clone(), Path("api-profile".to_string()), Json(json!({"enable": true}))).await;
        let check_opt = check_profile(app_state.clone(), Path("api-profile".to_string())).await.unwrap();
        assert_eq!(check_opt.options.0.len(), 1);

        // 5. Remove Option
        let _ =
            profile_remove_option(app_state.clone(), Path("api-profile".to_string()), Json(json!({"enable": true})))
                .await;
        let check_opt_rm = check_profile(app_state.clone(), Path("api-profile".to_string())).await.unwrap();
        assert!(check_opt_rm.options.0.is_empty());

        // 6. Delete Profile
        let res_del = delete_profile(app_state.clone(), Path("api-profile".to_string())).await;
        assert!(res_del.is_ok());
    }
}
