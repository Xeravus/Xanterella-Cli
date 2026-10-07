use std::{convert::Infallible, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    response::{
        Html, IntoResponse, Sse,
        sse::{Event, KeepAlive},
    },
    routing::{delete, get, post},
};
use serde_json::{Value, json};
use tokio_stream::Stream;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use xanterella_core::{Ping, Xanterella, xanterella::{EventState, EventFormat}, XanterellaInstall, prolyxena::Nixtractor};
use prolyxena::engine::lexer::vfs::*;

use crate::{ApiError, AppState, hosts::*, modules::*, profiles::*};

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health_check))
        .route("/ping/:ip", get(get_ping))
        .route("/stream", get(event_stream))
        .route("/extract", get(extract_configs))
        .route("/clear", get(clear))
        .route("/hosts", get(list_hosts).post(create_host))
        .route("/hosts/:hostname", get(check_host).delete(delete_host))
        .route("/hosts/:hostname/profiles", post(host_add_profile).delete(host_remove_profile))
        .route("/hosts/:hostname/option", post(host_add_option).delete(host_remove_option))
        .route("/profiles", get(list_profiles).post(create_profile))
        .route("/profiles/:name", get(check_profile).delete(delete_profile))
        .route("/profiles/:name/option", post(profile_add_option).delete(profile_remove_option))
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

pub async fn extract_configs(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Extract of Files".to_string(),
    });

    let mut fs_data = FsData::new("/home/cato/xanterella/config");
    fs_data.load().map_err(|err| {
        eprintln!("Extractions-Fehler: VFS Init Fehler: '{}'", err);
        ApiError::InternalError
    })?;

    let mut extractor = Nixtractor::new(&mut fs_data).await;

    let hosts = extractor.extract_hosts().await.map_err(|err| {
        eprintln!("Extractions-Fehler: Hosts: \n{}", err);
        ApiError::InternalError 
    })?;
    let profiles = extractor.extract_profiles().await.map_err(|err| {
        eprintln!("Extractions-Fehler: Profiles: \n{}", err);
        ApiError::InternalError 
    })?;
    let modules = extractor.extract_modules().await.map_err(|err| {
        eprintln!("Extractions-Fehler: Modules: \n{}", err);
        ApiError::InternalError 
    })?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: "Extraction of Files".to_string(),
    });

    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Synchronise Config with DB".to_string(),
    });

    for h in &hosts {
        if let Err(e) = state.db.add_host(&h.hostname, &h.ip, h.profiles.clone(), h.options.clone()).await {
            eprintln!("Sync-Fehler: Host '{}': {}", h.hostname, e);
        }
    }

    for p in &profiles {
        if let Err(e) = state.db.add_profile(&p.name, &p.dir, p.options.clone()).await {
            eprintln!("Sync-Fehler: Profile '{}': {}", p.name, e);
        }
    }

    for m in &modules {
        if let Err(e) = state.db.add_modul(&m.name, &m.desc, &m.category, m.options.clone()).await {
            eprintln!("Sync-Fehler: Modul '{}': {}", m.name, e);
        }
    }

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: "Synchronisation of Files".to_string(),
    });

    Ok(Json(json!({
        "status": "success",
        "message": format!("Synchronisiert: {} Hosts, {} Profile, {} Module", hosts.len(), profiles.len(), modules.len())
    })))
}

pub async fn clear(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Clearing DB".to_string(),
    });

    let clear = state.db.clear().await.map_err(|err| {
        eprintln!("Clearing-Fehler: {}", err);
        ApiError::InternalError
    })?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: "Cleared DB".to_string(),
    });
    Ok(Json(json!({
        "status": "success",
        "message": clear,
    })))
}
