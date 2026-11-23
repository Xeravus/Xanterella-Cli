use std::sync::Arc;
use xanterella_core::xanterella::RealCommandRunner;

use axum::{Json, extract::State};
use serde::Deserialize;
use serde_json::{Value, json};
use xanterella_core::{
    Xanterella,
    xanterella::{EventFormat, EventState},
};

use crate::{ApiError, AppState};

#[derive(Deserialize)]
pub struct DeployPayload {
    pub hosts: Vec<String>,
}

pub async fn deploy_all(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Deploying all".to_string(),
    });

    tokio::spawn(async move {
        let deploy = tokio::task::spawn_blocking(|| {
            let mut xanterella = Xanterella::new(RealCommandRunner);
            xanterella.colmena_apply::<&str>(&[])
        })
        .await;

        match deploy {
            Ok(Ok(_)) => {
                let _ = state.tx.send(EventFormat {
                    state: EventState::Finish,
                    step: "Deployed all".to_string(),
                });
            }
            Ok(Err(err)) => {
                let _ = state.tx.send(EventFormat {
                    state: EventState::Failed,
                    step: format!("Deployment Failed: {:?}", err),
                });
            }
            Err(_) => {
                let _ = state.tx.send(EventFormat {
                    state: EventState::Failed,
                    step: "Internal Error".to_string(),
                });
            }
        }
    });

    Ok(Json(json!({
        "status": "success",
    })))
}

pub async fn deploy_selected(
    State(state): State<Arc<AppState>>, Json(payload): Json<DeployPayload>,
) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Deploying selected".to_string(),
    });

    let paths = state.paths.clone();
    tokio::spawn(async move {
        let deploy = tokio::task::spawn_blocking(move || {
            let mut xanterella = Xanterella::new(RealCommandRunner);
            xanterella.set_config(paths);
            xanterella.colmena_apply(&payload.hosts)
        })
        .await;

        match deploy {
            Ok(Ok(_)) => {
                let _ = state.tx.send(EventFormat {
                    state: EventState::Finish,
                    step: "Deployed selected".to_string(),
                });
            }
            Ok(Err(err)) => {
                let _ = state.tx.send(EventFormat {
                    state: EventState::Failed,
                    step: format!("Deployment Failed: {:?}", err),
                });
            }
            Err(_) => {
                let _ = state.tx.send(EventFormat {
                    state: EventState::Failed,
                    step: "Internal Error".to_string(),
                });
            }
        }
    });

    Ok(Json(json!({
        "status": "success",
    })))
}
