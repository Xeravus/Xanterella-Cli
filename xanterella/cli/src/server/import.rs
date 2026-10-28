use std::process;

use prolyxena::engine::lexer::vfs::FsData;
use serde::Serialize;
use xanterella_core::prolyxena::{CreateHost, CreateModul, CreateProfile, CreateTemplate, Nixtractor};

trait ApiSend: Serialize {
    fn endpoint(&self) -> &str;
}

impl ApiSend for CreateHost {
    fn endpoint(&self) -> &str {
        "/hosts"
    }
}

impl ApiSend for CreateProfile {
    fn endpoint(&self) -> &str {
        "/profiles"
    }
}

impl ApiSend for CreateModul {
    fn endpoint(&self) -> &str {
        "/modules"
    }
}

impl ApiSend for CreateTemplate {
    fn endpoint(&self) -> &str {
        "/templates"
    }
}

pub async fn import_all(flake: &str, url: &str) {
    import_hosts(&flake, &url).await;
    import_profiles(&flake, &url).await;
    import_modules(&flake, &url).await;
    import_templates(&flake, &url).await;
}

pub async fn import_hosts(flake: &str, url: &str) {
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

pub async fn import_profiles(flake: &str, url: &str) {
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

pub async fn import_modules(flake: &str, url: &str) {
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

pub async fn import_templates(flake: &str, url: &str) {
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

async fn send_to_api<I: ApiSend>(data: Vec<I>, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let full_url = format!("http://{}{}", url, &data.first().expect("Nichts zu versenden").endpoint());
    for i in data {
        let response = client.post(&full_url).json(&i).send().await?;

        if !response.status().is_success() {
            println!("{}", response.text().await?);
            process::exit(2);
        }
    }
    Ok(())
}
