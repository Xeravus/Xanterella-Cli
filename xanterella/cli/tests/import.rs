#[path = "../../tests/support/flake.rs"]
mod flake;
#[path = "../../tests/support/mod.rs"]
mod support;

use mockito::{Matcher, Server};
use serde_json::json;
use xanterella_cli::{execute::execute_extract};
use xanterella_cli::server::import::import;

#[tokio::test]
async fn imports_each_resource_from_a_real_flake() {
    let dir = support::TestDir::new();
    flake::write_flake(dir.path());
    let path = dir.path().to_str().unwrap();
    execute_extract(path).await;
    let mut server = Server::new_async().await;
    let mut expected = Vec::new();
    for (endpoint, fields) in [
        ("/hosts", json!({"hostname":"worker","remote_deploy":true})),
        ("/profiles", json!({"name":"worker","dir":"apps"})),
        ("/modules", json!({"name":"editor","category":"tools"})),
        ("/templates", json!({"path":"flake.nix"})),
        ("/templates", json!({"path":"flake-modules/packages.nix"})),
        ("/raw", json!({"name":"icon.bin","path":"icon.bin","content":[255,0,1],"readable":false})),
    ] {
        expected.push(server.mock("POST", endpoint).match_body(Matcher::PartialJson(fields)).create_async().await);
    }
    import(path, &server.host_with_port(), &false, &false, &false, &false, &false, &true).await;
    for request in expected {
        request.assert_async().await;
    }
}
