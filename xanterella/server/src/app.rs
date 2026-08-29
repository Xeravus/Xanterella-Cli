use std::sync::Arc;
use std::convert::Infallible;

use tokio_stream::Stream;
use axum::{
    Json, Router,
    extract::{Path, State},
    response::{IntoResponse, Sse, sse::{KeepAlive, Event}},
    routing::{get},
};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use xanterella_core::{Ping, Xanterella, XanterellaInstall, xanterella::{EventFormat, EventState}, db::DBHost};

use crate::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateHost {
    pub hostname: String,
    pub ip: String,
}

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ping/:ip", get(get_ping))
        .route("/stream", get(event_stream))
        .route("/hosts", get(list_hosts).post(create_host))
        .route("/hosts/:hostname", get(check_host).delete(delete_host))
        .with_state(state)
}

pub async fn event_stream(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| {
        match msg {
            Ok(event) => {
                let json_data = serde_json::to_string(&event).unwrap_or_default();
                Some(Ok::<_, Infallible>(Event::default().data(json_data)))
            }
            Err(_) => {
                None
            }
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn get_ping(State(state): State<Arc<AppState>>, Path(ip): Path<String>) -> Result<Json<Value>, ApiError> {
    let xanterella = Xanterella::new();
    let mut install = XanterellaInstall::new(xanterella);
    install.xanterella.set_sender(state.tx.clone());
    install.set_ip(&ip);

    let result = tokio::task::spawn_blocking(move || install.ping()).await.map_err(|_| ApiError::InternalError)?;

    match result {
        Ok(_) => Ok(Json(json!({
            "status": "ok",
            "ip": ip,
        }))),
        Err(_) => Err(ApiError::InternalError),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
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
