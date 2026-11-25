use mockito::{Matcher::Json, Server};
use serde_json::json;
use xanterella_cli::server::requests::{
    request_check, request_clear, request_deployment, request_generate, request_wipe,
};

#[tokio::test]
async fn test_request_clear_success() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/clear").with_status(200).create_async().await;
    let url = server.host_with_port();
    let result = request_clear(&url).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_clear_failed() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/clear").with_status(500).create_async().await;
    let url = server.host_with_port();
    let result = request_clear(&url).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_generate_success() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/generate").with_status(200).create_async().await;
    let url = server.host_with_port();
    let result = request_generate(&url).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_generate_failed() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/generate").with_status(500).create_async().await;
    let url = server.host_with_port();
    let result = request_generate(&url).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_wipe_success() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/wipe").with_status(200).create_async().await;
    let url = server.host_with_port();
    let result = request_wipe(&url).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_wipe_failed() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/wipe").with_status(500).create_async().await;
    let url = server.host_with_port();
    let result = request_wipe(&url).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_check_success() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/config/check").with_status(200).create_async().await;
    let url = server.host_with_port();
    let result = request_check(&url).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_check_failed() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/config/check").with_status(500).create_async().await;
    let url = server.host_with_port();
    let result = request_check(&url).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_deployment_success() {
    let mut server = Server::new_async().await;

    let _mock = server
        .mock("POST", "/deploy")
        .match_body(Json(json!({
            "hosts": ["hosts"]
        })))
        .with_status(200)
        .create_async()
        .await;
    let url = server.host_with_port();
    let result = request_deployment(&url, &["hosts"]).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_deployment_failed() {
    let mut server = Server::new_async().await;

    let _mock = server
        .mock("POST", "/deploy")
        .match_body(Json(json!({
            "hosts": ["hosts"]
        })))
        .with_status(500)
        .create_async()
        .await;
    let url = server.host_with_port();
    let result = request_deployment(&url, &["hosts"]).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_request_deployment_success_all() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/deploy").with_status(200).create_async().await;
    let url = server.host_with_port();
    let result = request_deployment::<&str>(&url, &[]).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_request_deployment_failed_all() {
    let mut server = Server::new_async().await;

    let _mock = server.mock("GET", "/deploy").with_status(500).create_async().await;
    let url = server.host_with_port();
    let result = request_deployment::<&str>(&url, &[]).await;

    assert!(result.is_err());
}
