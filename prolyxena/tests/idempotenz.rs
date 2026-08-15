use std::fs;
use std::path::PathBuf;
use prolyxena::engine::core::*;
use prolyxena::engine::lexer::primitives::ParsePrimitves;
use prolyxena::engine::formater::sort::Sort;
use prolyxena::engine::formater::flattening::Flattening;
use prolyxena::engine::formater::core::Format;

fn idempotenz_format_assert(file: &str) {
    let mut inital_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    inital_path.push(format!("tests/fixtures/format/initial/{}.nix", file));
    expected_path.push(format!("tests/fixtures/format/expected/{}.nix", file));

    let inital_code = fs::read_to_string(&inital_path).expect("Input-Datei fehlt");
    let expected_code = fs::read_to_string(&expected_path).expect("Expected-Datei fehlt");

    let mut lexer = Lexer::new(&inital_code, inital_path.to_string_lossy().to_string());
    let mut ast = lexer.parse_single_value().expect("Parsing des Inputs schlug fehl");
    ast.expand();
    ast.sort_ast();
    let first_format = ast.format_nix(0);

    assert_eq!( first_format, expected_code, "Fehler beim Formatieren der Datei: {}", file);

    let mut lexer_idempotent = Lexer::new(&first_format, "idempotent.nix".to_string());
    let mut ast_idempotent = lexer_idempotent.parse_single_value().expect("Parsing des formatierten Codes schlug fehl");
    ast_idempotent.expand();
    ast_idempotent.sort_ast();
    let second_format = ast_idempotent.format_nix(0);

    assert_eq!( first_format, second_format, "Formatter ist nicht idempotent für Test: {}!", file);
    assert_eq!( second_format, expected_code, "Formatter ist nicht idempotent für Test: {}!", file);
}

#[test]
fn int_idempotenz_format_flake() {
    idempotenz_format_assert("flake");
}

#[test]
fn int_idempotenz_format_github_runner() {
    idempotenz_format_assert("github-runner");
}

#[test]
fn int_idempotenz_format_hardware_configuration() {
    idempotenz_format_assert("hardware-configuration");
}

#[test]
fn int_idempotenz_format_matrix_server() {
    idempotenz_format_assert("matrix-server");
}

#[test]
fn int_idempotenz_format_monitoring() {
    idempotenz_format_assert("monitoring");
}

#[test]
fn int_idempotenz_format_shells() {
    idempotenz_format_assert("shells");
}

#[test]
fn int_idempotenz_format_tailscale() {
    idempotenz_format_assert("tailscale");
}
