use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::{Value, json};
use tokio::task::spawn_blocking;
use xanterella_core::{
    Xanterella,
    xanterella::{EventFormat, EventState, RealCommandRunner},
};

use crate::{ApiError, AppState};

pub async fn check_config(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Checking Config".to_string(),
    });

    let paths = state.paths.clone();
    spawn_blocking(move || {
        let mut xanterella = Xanterella::new(RealCommandRunner);
        xanterella.set_config(paths);
        xanterella.check_nix_flake().map_err(|err| {
            eprintln!("Config-Fehler: Checking: {:#?}", err);
            ApiError::InternalError
        })?;
        Ok::<(), ApiError>(())
    })
    .await
    .map_err(|_err| ApiError::InternalError)??;
    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: "Checked Config".to_string(),
    });
    Ok(Json(json!({
                "status": "ok",
                "check": "completed",
    })))
}
