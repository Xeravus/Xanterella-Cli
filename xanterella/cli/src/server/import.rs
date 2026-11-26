use std::process;

use prolyxena::engine::lexer::vfs::FsData;
use serde::Serialize;
use xanterella_core::prolyxena::{CreateHost, CreateModul, CreateProfile, CreateRaw, CreateTemplate, Nixtractor};

trait ApiSend: Serialize {
    const ENDPOINT: &'static str;
}

impl ApiSend for CreateHost {
    const ENDPOINT: &'static str = "/hosts";
}

impl ApiSend for CreateProfile {
    const ENDPOINT: &'static str = "/profiles";
}

impl ApiSend for CreateModul {
    const ENDPOINT: &'static str = "/modules";
}

impl ApiSend for CreateTemplate {
    const ENDPOINT: &'static str = "/templates";
}

impl ApiSend for CreateRaw {
    const ENDPOINT: &'static str = "/raw";
}

#[allow(clippy::too_many_arguments)]
pub async fn import(
    flake: &str, url: &str, hosts: &bool, profiles: &bool, modules: &bool, templates: &bool, raw: &bool, all: &bool,
) {
    if !*hosts && !*profiles && !*modules && !*templates && !*raw && !*all {
        import_hosts(flake, url).await;
        import_profiles(flake, url).await;
        import_modules(flake, url).await;
        import_templates(flake, url).await;
        import_raw(flake, url).await;
    } else {
        if *hosts || *all {
            import_hosts(flake, url).await;
        }

        if *profiles || *all {
            import_profiles(flake, url).await;
        }

        if *modules || *all {
            import_modules(flake, url).await;
        }

        if *templates || *all {
            import_templates(flake, url).await;
        }

        if *raw || *all {
            import_raw(flake, url).await;
        }
    }
}

pub(crate) async fn import_hosts(flake: &str, url: &str) {
    let mut prolyxena = FsData::new(flake);
    if let Err(e) = prolyxena.load() {
        eprintln!("Fehler: {}", e);
        process::exit(1);
    }

    let extractor = Nixtractor::new(&mut prolyxena);
    let hosts = match extractor.await.extract_hosts().await {
        Ok(h) => h,
        Err(err) => panic!("Fehler: Error while extracting the Hosts: {}", err),
    };
    println!("Hosts OK");
    match send_to_api(hosts, url).await {
        Ok(_) => println!("Hosts Sent"),
        Err(err) => println!("Fehler beim senden der Hosts: {:#?}", err),
    }
}

pub(crate) async fn import_profiles(flake: &str, url: &str) {
    let mut prolyxena = FsData::new(flake);
    if let Err(e) = prolyxena.load() {
        eprintln!("Fehler: {}", e);
        process::exit(1);
    }

    let extractor = Nixtractor::new(&mut prolyxena);
    let profiles = match extractor.await.extract_profiles().await {
        Ok(h) => h,
        Err(err) => panic!("Fehler: Error while extracting the Profiles: {}", err),
    };
    println!("Profiles OK");
    match send_to_api(profiles, url).await {
        Ok(_) => println!("Profiles Sent"),
        Err(err) => println!("Fehler beim senden der Profiles: {:#?}", err),
    }
}

pub(crate) async fn import_modules(flake: &str, url: &str) {
    let mut prolyxena = FsData::new(flake);
    if let Err(e) = prolyxena.load() {
        eprintln!("Fehler: {}", e);
        process::exit(1);
    }

    let extractor = Nixtractor::new(&mut prolyxena);
    let modules = match extractor.await.extract_modules().await {
        Ok(h) => h,
        Err(err) => panic!("Fehler: Error while extracting the Modules: {}", err),
    };
    println!("Modules OK");
    match send_to_api(modules, url).await {
        Ok(_) => println!("Modules Sent"),
        Err(err) => println!("Fehler beim senden der Modules: {:#?}", err),
    }
}

pub(crate) async fn import_templates(flake: &str, url: &str) {
    let mut prolyxena = FsData::new(flake);
    if let Err(e) = prolyxena.load() {
        eprintln!("Fehler: {}", e);
        process::exit(1);
    }

    let extractor = Nixtractor::new(&mut prolyxena);
    let modules = match extractor.await.extract_templates().await {
        Ok(h) => h,
        Err(err) => panic!("Fehler: Error while extracting the Templates: {}", err),
    };
    println!("Templates OK");
    match send_to_api(modules, url).await {
        Ok(_) => println!("Templates Sent"),
        Err(err) => println!("Fehler beim senden der Templates: {:#?}", err),
    }
}

pub(crate) async fn import_raw(flake: &str, url: &str) {
    let mut prolyxena = FsData::new(flake);

    let extractor = Nixtractor::new(&mut prolyxena);
    let modules = match extractor.await.extract_raw().await {
        Ok(h) => h,
        Err(err) => panic!("Fehler: Error while extracting the Raw Files: {}", err),
    };
    println!("Raw Files OK");
    match send_to_api(modules, url).await {
        Ok(_) => println!("Raw Files Sent"),
        Err(err) => println!("Fehler beim senden der Raw Files: {:#?}", err),
    }
}

async fn send_to_api<I: ApiSend>(data: Vec<I>, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if data.is_empty() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let full_url = format!("http://{}{}", url, I::ENDPOINT);
    for i in data {
        let response = client.post(&full_url).json(&i).send().await?;

        if !response.status().is_success() {
            println!("{}", response.text().await?);
            process::exit(2);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send_to_api_no_data() {
        let result = send_to_api::<CreateHost>(Vec::new(), "127.0.0.1");

        assert!(result.await.is_ok());
    }
}
