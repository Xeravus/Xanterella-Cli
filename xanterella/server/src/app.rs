use std::{convert::Infallible, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{
        Html, IntoResponse, Sse,
        sse::{Event, KeepAlive},
    },
    routing::get,
};
use serde_json::{Value, json};
use tokio_stream::Stream;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use xanterella_core::{Ping, Xanterella, XanterellaInstall};

use crate::{ApiError, AppState, hosts::*, modules::*};

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health_check))
        .route("/ping/:ip", get(get_ping))
        .route("/stream", get(event_stream))
        .route("/hosts", get(list_hosts).post(create_host))
        .route("/hosts/:hostname", get(check_host).delete(delete_host))
        .route("/modules", get(list_modules).post(create_modul))
        .route("/modules/:name", get(check_modul).delete(delete_modul))
        .with_state(state)
}

pub async fn dashboard() -> Html<&'static str> {
    Html(include_str!("../website/index.html"))
}

pub async fn event_stream(State(state): State<Arc<AppState>>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(event) => {
            let json_data = serde_json::to_string(&event).unwrap_or_default();
            Some(Ok::<_, Infallible>(Event::default().data(json_data)))
        }
        Err(_) => None,
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
