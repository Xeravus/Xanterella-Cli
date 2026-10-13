use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use serde_json::{Value, json};
use xanterella_core::{
    database::db::DBHost,
    prolyxena::CreateHost,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

pub async fn create_host(
    State(state): State<Arc<AppState>>, Json(payload): Json<CreateHost>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Host '{}'", payload.hostname),
    });
    state
        .db
        .add_host(&payload.hostname, &payload.ip, payload.profiles.clone(), payload.options)
        .await
        .map_err(|_| ApiError::InternalError)?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Host '{}' added", payload.hostname),
    });

    Ok(Json(json!({
        "status": "success",
        "hostname": payload.hostname
    })))
}

pub async fn delete_host(
    State(state): State<Arc<AppState>>, Path(hostname): Path<String>,
) -> Result<Json<Value>, ApiError> {
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

pub async fn check_host(
    State(state): State<Arc<AppState>>, Path(hostname): Path<String>,
) -> Result<Json<DBHost>, ApiError> {
    let host = state.db.get_host(&hostname).await.map_err(|_| ApiError::InternalError)?;
    match host {
        Some(h) => Ok(Json(h)),
        None => Err(ApiError::NotFound),
    }
}

pub async fn host_add_profile(
    State(state): State<Arc<AppState>>, Path(hostname): Path<String>, Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Profile to Host '{}'", hostname),
    });
    state.db.host_add_profile(&hostname, payload).await.map_err(|_| ApiError::InternalError)?;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Added Profile to Host '{}'", hostname),
    });
    Ok(Json(json!({
        "status": "success",
        "hostname": hostname,
    })))
}

pub async fn host_remove_profile(
    State(state): State<Arc<AppState>>, Path(hostname): Path<String>, Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Remove Profile from Host '{}'", hostname),
    });
    state.db.host_remove_profile(&hostname, payload).await.map_err(|_| ApiError::InternalError)?;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Removed Profile from Host '{}'", hostname),
    });
    Ok(Json(json!({
        "status": "success",
        "hostname": hostname,
    })))
}

pub async fn host_add_option(
    State(state): State<Arc<AppState>>, Path(hostname): Path<String>, Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Add Option to Host '{}'", hostname),
    });
    state.db.host_add_option(&hostname, payload).await.map_err(|_| ApiError::InternalError)?;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Added Option to Host '{}'", hostname),
    });
    Ok(Json(json!({
        "status": "success",
        "hostname": hostname,
    })))
}

pub async fn host_remove_option(
    State(state): State<Arc<AppState>>, Path(hostname): Path<String>, Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: format!("Remove Option from Host '{}'", hostname),
    });
    state.db.host_remove_option(&hostname, payload).await.map_err(|_| ApiError::InternalError)?;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: format!("Removed Option from Host '{}'", hostname),
    });
    Ok(Json(json!({
        "status": "success",
        "hostname": hostname,
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
    async fn test_api_hosts_crud() {
        let state = test_state().await;
        let app_state = State(state.clone());

        // 1. Create Host
        let payload = Json(xanterella_core::prolyxena::CreateHost {
            hostname: "api-host".to_string(),
            ip: "10.0.0.1".to_string(),
            profiles: vec![],
            options: vec![],
        });
        let res_create = create_host(app_state.clone(), payload).await;
        assert!(res_create.is_ok());

        // 2. List Hosts
        let list = list_hosts(app_state.clone()).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].hostname, "api-host");

        // 3. Check Host
        let check = check_host(app_state.clone(), Path("api-host".to_string())).await.unwrap();
        assert_eq!(check.hostname, "api-host");

        // 4. Add & Remove Profile
        let _ = host_add_profile(app_state.clone(), Path("api-host".to_string()), Json(json!({"path": "test"}))).await;
        let check_prof = check_host(app_state.clone(), Path("api-host".to_string())).await.unwrap();
        assert_eq!(check_prof.profiles.0.len(), 1);

        let _ =
            host_remove_profile(app_state.clone(), Path("api-host".to_string()), Json(json!({"path": "test"}))).await;
        let check_prof_rm = check_host(app_state.clone(), Path("api-host".to_string())).await.unwrap();
        assert!(check_prof_rm.profiles.0.is_empty());

        // 5. Add & Remove Option
        let _ = host_add_option(app_state.clone(), Path("api-host".to_string()), Json(json!({"port": 80}))).await;
        let check_opt = check_host(app_state.clone(), Path("api-host".to_string())).await.unwrap();
        assert_eq!(check_opt.options.0.len(), 1);

        let _ = host_remove_option(app_state.clone(), Path("api-host".to_string()), Json(json!({"port": 80}))).await;
        let check_opt_rm = check_host(app_state.clone(), Path("api-host".to_string())).await.unwrap();
        assert!(check_opt_rm.options.0.is_empty());

        // 6. Delete Host
        let res_del = delete_host(app_state.clone(), Path("api-host".to_string())).await;
        assert!(res_del.is_ok());

        // 7. Verify Deletion (Not Found)
        let not_found = check_host(app_state.clone(), Path("api-host".to_string())).await;
        assert!(matches!(not_found, Err(ApiError::NotFound)));
    }
}
