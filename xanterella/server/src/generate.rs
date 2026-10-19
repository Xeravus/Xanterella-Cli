use std::{fmt::Write, sync::Arc, collections::HashMap};
use prolyxena::engine::formater::write::Write as WalkTree;

use axum::{Json, extract::State};
use prolyxena::engine::{generator::generate::Modify, lexer::vfs::FsData};
use serde_json::{Value, json};
use xanterella_core::xanterella::{EventFormat, EventState};

use crate::{ApiError, AppState, hosts::HostsResponse, modules::ModulesResponse, profiles::ProfilesResponse};

pub async fn generate_config(State(state): State<Arc<AppState>>) -> Result<Json<Value>, ApiError> {
    let _ = state.tx.send(EventFormat {
        state: EventState::Run,
        step: "Generating Config".to_string(),
    });

    let hosts = match state.db.list_hosts().await {
        Ok(h) => h,
        Err(err) => {
            eprintln!("Generation-Fehler: Hosts konnten nicht aus der DB abgerufen werden: {}", err);
            return Err(ApiError::InternalError);
        }
    };
    let hosts_len = hosts.len();
    let formated_hosts: Vec<HostsResponse> = hosts.into_iter().map(HostsResponse::from).collect();

    let profiles = match state.db.list_profiles().await {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Generation-Fehler: Profiles konnten nicht aus der DB abgerufen werden: {}", err);
            return Err(ApiError::InternalError);
        }
    };
    let profiles_len = profiles.len();
    let formated_profiles: Vec<ProfilesResponse> = profiles.into_iter().map(ProfilesResponse::from).collect();

    let modules = match state.db.list_modules().await {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Generation-Fehler: Module konnten nicht aus der DB abgerufen werden: {}", err);
            return Err(ApiError::InternalError);
        }
    };
    let modules_len = modules.len();
    let formated_modules: Vec<ModulesResponse> = modules.into_iter().map(ModulesResponse::from).collect();

    let mut prolyxena = FsData::new(&state.config.target_directory);
    let mut name = String::with_capacity(64);

    for i in formated_hosts {
        name.clear();
        write!(&mut name, "hosts/{}/configuration.nix", i.hostname).unwrap();
        prolyxena.generate_file(&name, i.config).map_err(|err| {
            eprintln!(
                "Generation-Fehler: Configuration.nix konnte für den Host: {} nciht erstellt werden: {}",
                i.hostname, err
            );
            ApiError::InternalError
        })?;
        name.clear();
        write!(&mut name, "hosts/{}/hardware-configuration.nix", i.hostname).unwrap();
        prolyxena.generate_file(&name, i.hardware).map_err(|err| {
            eprintln!(
                "Generation-Fehler: Hardware-configuration.nix konnte für den Host: {} nciht erstellt werden: {}",
                i.hostname, err
            );
            ApiError::InternalError
        })?;
    }

    for i in formated_profiles {
        name.clear();
        if &i.dir == "base" {
            write!(&mut name, "profiles/{}.nix", i.name).unwrap();
        } else {
            write!(&mut name, "profiles/{}/{}.nix", i.dir, i.name).unwrap()
        }

        prolyxena.generate_file(&name, i.options).map_err(|err| {
            eprintln!("Generation-Fehler: {} konnte nicht als Profile erstellt werden: {}", name, err);
            ApiError::InternalError 
        })?;
    }

    let mut modules_for_default: HashMap<String, Vec<String>> = HashMap::with_capacity(formated_modules.len());
    for i in formated_modules {
        name.clear();
        if i.category.is_empty() {
            write!(&mut name, "modules/{}.nix", i.name).unwrap();
        } else {
            write!(&mut name, "modules/{}/{}.nix", i.category, i.name).unwrap();
        }

        prolyxena.generate_file(&name, i.config).map_err(|err| {
            eprintln!("Generation-Fehler: {} konnte nicht als Modul erstellt werden: {}", name, err);
            ApiError::InternalError
        })?;
        modules_for_default.entry(i.category).or_default().push(i.name);
    }

    let mut root_imports = Vec::new();
    let mut file_path = String::with_capacity(64);

    for (category, module_names) in modules_for_default {
        if category == "base" {
            for i in module_names {
                root_imports.push(format!("./{}.nix", i));
            }
        } else {
            root_imports.push(format!("./{}", category));
            let mut category_imports = String::new();
            for i in module_names {
                category_imports.push_str(&format!("./{}.nix ", i));
            }

            let category_default_content = format!("{{imports=[{}];}}", category_imports);

            file_path.clear();
            write!(&mut file_path, "modules/{}/default.nix", category).unwrap();
            prolyxena.generate_file(&file_path, category_default_content).map_err(|err| {
                eprintln!("Generation-Fehler: {} konnte nicht als default.nix im Modul Ordner erstellt werden: {}", file_path, err);
                ApiError::InternalError 
            })?;
        }
    }

    let root_default_content = format!("{{imports=[ {} ];}}", root_imports.join(" "));
    file_path.clear();
    write!(&mut file_path, "modules/default.nix").unwrap();
    prolyxena.generate_file(&file_path, root_default_content).map_err(|err| {
        eprintln!("Generation-Fehler: {} konnte nciht als default.nix im Modul Ordner erstellt werden: {}", file_path, err);
        ApiError::InternalError
    })?;

    let _ = state.tx.send(EventFormat {
        state: EventState::Finish,
        step: "Generated Config".to_string(),
    });
    match prolyxena.walk_tree() {
        Err(err) => {
            eprintln!("Generation-Fehler: Konnte die Configs nicht erstellen: {}", err);
            return Err(ApiError::InternalError);
        }
        _ => { },
    };
    Ok(Json(json!({
        "status": "success",
        "hosts": hosts_len,
        "profiles": profiles_len,
        "modules": modules_len,
    })))
}
