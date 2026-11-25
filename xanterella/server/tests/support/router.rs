#[path = "../../../tests/support/flake.rs"]
mod flake;
#[path = "../../../tests/support/mod.rs"]
mod support;

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt;

use crate::{app::create_app, test_helper::tests_utils::test_state};

async fn request(router: &Router, method: Method, path: &str, payload: Option<Value>, status: StatusCode) -> Value {
    let mut request = Request::builder().method(method).uri(path);
    let body = match payload {
        Some(value) => {
            request = request.header("content-type", "application/json");
            Body::from(value.to_string())
        }
        None => Body::empty(),
    };
    let response = router.clone().oneshot(request.body(body).unwrap()).await.unwrap();
    assert_eq!(response.status(), status, "{path}");
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn extracts_flake_into_sqlite_and_clears_all_resources() {
    let dir = support::TestDir::new();
    flake::write_flake(dir.path());
    let mut state = test_state().await;
    std::sync::Arc::get_mut(&mut state).unwrap().config.target_directory = dir.path().display().to_string();
    let router = create_app(state);
    let result = request(&router, Method::GET, "/extract", None, StatusCode::OK).await;
    assert_eq!(result["status"], "success");
    for (path, key, value) in [
        ("/hosts/worker", "system", "x86_64-linux"),
        ("/profiles/worker", "dir", "apps"),
        ("/modules/editor", "category", "tools"),
        ("/templates/packages.nix", "path", "flake-modules/packages.nix"),
    ] {
        assert_eq!(request(&router, Method::GET, path, None, StatusCode::OK).await[key], value);
    }
    let result = request(&router, Method::GET, "/clear", None, StatusCode::OK).await;
    assert_eq!(result["message"], json!({"Hosts":1,"Modules":1,"Templates":2,"Profiles":1,"Raw":0}));
    for path in ["/hosts", "/modules", "/profiles", "/templates"] {
        assert_eq!(request(&router, Method::GET, path, None, StatusCode::OK).await["len"], 0);
    }
}

#[tokio::test]
async fn raw_files_round_trip_preserves_bytes_and_masks_binary_list_content() {
    let router = create_app(test_state().await);
    let payload = json!({"name":"icon","path":"images/icon.bin","readable":false,"content":[255,0,1]});
    request(&router, Method::POST, "/raw", Some(payload.clone()), StatusCode::OK).await;
    let item = request(&router, Method::GET, "/raw/icon", None, StatusCode::OK).await;
    assert_eq!(item["content"], payload["content"]);
    assert_eq!(item["path"], payload["path"]);
    let list = request(&router, Method::GET, "/raw", None, StatusCode::OK).await;
    assert_eq!(list["len"], 1);
    assert_eq!(list["raw"][0]["content"], " --- Cryptic Text --- ");
    request(&router, Method::DELETE, "/raw/icon", None, StatusCode::OK).await;
    for method in [Method::GET, Method::DELETE] {
        let error = request(&router, method, "/raw/icon", None, StatusCode::NOT_FOUND).await;
        assert_eq!(error["error"], "Data not found");
    }
}

#[tokio::test]
async fn templates_round_trip_through_router_and_database() {
    let router = create_app(test_state().await);
    let payload = json!({"name":"packages","path":"flake-modules/packages.nix","config":[]});
    request(&router, Method::POST, "/templates", Some(payload.clone()), StatusCode::OK).await;
    let list = request(&router, Method::GET, "/templates", None, StatusCode::OK).await;
    assert_eq!(list["len"], 1);
    assert_eq!(list["templates"][0]["path"], payload["path"]);
    assert_eq!(list["templates"][0]["config"], "");
    request(&router, Method::DELETE, "/templates/packages", None, StatusCode::OK).await;
    for method in [Method::GET, Method::DELETE] {
        request(&router, method, "/templates/packages", None, StatusCode::NOT_FOUND).await;
    }
}
