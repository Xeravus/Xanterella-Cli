use reqwest::Client;
use serde_json::json;

pub async fn request_clear(url: &String) {
    let client = Client::new();
    let response = client.get(format!("http://{}/clear", url)).send().await.expect("Fehler beim senden der Request");
    if response.status().is_success() {
        println!("DB geleert");
    } else {
        eprintln!("Request-Fehler: Konnte DB nicht leeren: {}", response.status());
    }
}

pub async fn request_generate(url: &String) {
    let client = Client::new();
    let response = client.get(format!("http://{}/generate", url)).send().await.expect("Fehler beim senden der Request");
    if response.status().is_success() {
        println!("Config generiert");
    } else {
        eprintln!("Request-Fehler: Konnte die Config nicht generieren: {}", response.status());
    }
}

pub async fn request_wipe(url: &String) {
    let client = Client::new();
    let response = client.get(format!("http://{}/wipe", url)).send().await.expect("Fehler beim senden der Request");
    if response.status().is_success() {
        println!("Config gelöscht");
    } else {
        eprintln!("Request-Fehler: Konnte den Config Ordner nicht löschen: {}", response.status());
    }
}

pub async fn request_check(url: &String) {
    let client = Client::new();
    let response =
        client.get(format!("http://{}/config/check", url)).send().await.expect("Fehler beim senden der Request");
    if response.status().is_success() {
        println!("Config gecheckt");
    } else {
        eprintln!("Request-Fehler: Config ist nicht valide: {}", response.status());
    }
}

pub async fn request_deployment<T: AsRef<str>>(url: &String, hosts: &[T]) {
    let client = Client::new();
    let base_url = format!("http://{}/deploy", url);
    let response = if hosts.is_empty() {
        println!("Sende Deployment für alle Hosts");
        client.get(base_url).send().await.expect("Fehler beim senden der Request")
    } else {
        let hosts_vec: Vec<&str> = hosts.iter().map(|s| s.as_ref()).collect();
        println!("Sende Deployment für Hosts: {:?}", hosts_vec);
        let payload = json!({
            "hosts": hosts_vec
        });

        client.post(base_url).json(&payload).send().await.expect("Fehler beim senden der Request")
    };

    if response.status().is_success() {
        println!("Deployment gestartet");
    } else {
        eprintln!("Request-Fehler: Deployment konnte nicht gestartet werden: {}", response.status());
    }
}
