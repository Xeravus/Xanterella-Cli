

pub async fn request_clear(url: &String) {
    let client = reqwest::Client::new();
    let response = client.get(format!("http://{}/clear", url)).send().await.expect("Fehler beim senden der Clear Request");
    if response.status().is_success() {
        println!("DB geleert");
    } else {
        eprintln!("Request-Fehler: Konnte DB nicht leeren: {}", response.status());
    }
}

pub async fn request_generate(url: &String) {
    let client = reqwest::Client::new();
    let response = client.get(format!("http://{}/generate", url)).send().await.expect("Fehler beim senden der Clear Request");
    if response.status().is_success() {
        println!("Config generiert");
    } else {
        eprintln!("Request-Fehler: Konnte die Config nicht generieren: {}", response.status());
    }
}
