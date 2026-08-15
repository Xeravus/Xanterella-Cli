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

    assert_ne!(inital_code, expected_code, "Dateien('{}' & '{}') sind gleich. Dies darf nicht sein, damit ein Idempotenz test wirkvoll sein kann.", inital_path.display(), expected_path.display());

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

fn idempotenz_expand_assert(file: &str) {
    let mut inital_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    inital_path.push(format!("tests/fixtures/expand/initial/{}.nix", file));
    expected_path.push(format!("tests/fixtures/expand/expected/{}.nix", file));

    let inital_code = fs::read_to_string(&inital_path).expect("Input-Datei fehlt");
    let expected_code = fs::read_to_string(&expected_path).expect("Expected-Datei fehlt");

    assert_ne!(inital_code, expected_code, "Dateien('{}' & '{}') sind gleich. Dies darf nicht sein, damit ein Idempotenz test wirkvoll sein kann.", inital_path.display(), expected_path.display());

    let mut lexer = Lexer::new(&inital_code, inital_path.to_string_lossy().to_string());
    let mut ast = lexer.parse_single_value().expect("Parsing des Inputs schlug fehl");
    ast.expand();
    let first_format = ast.format_nix(0);

    assert_eq!( first_format, expected_code, "Fehler beim Formatieren der Datei: {}", file);

    let mut lexer_idempotent = Lexer::new(&first_format, "idempotent.nix".to_string());
    let mut ast_idempotent = lexer_idempotent.parse_single_value().expect("Parsing des formatierten Codes schlug fehl");
    ast_idempotent.expand();
    let second_format = ast_idempotent.format_nix(0);

    assert_eq!( first_format, second_format, "Formatter ist nicht idempotent für Test: {}!", file);
    assert_eq!( second_format, expected_code, "Formatter ist nicht idempotent für Test: {}!", file);
}

fn idempotenz_flatten_assert(file: &str) {
    let mut inital_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    inital_path.push(format!("tests/fixtures/flatten/initial/{}.nix", file));
    expected_path.push(format!("tests/fixtures/flatten/expected/{}.nix", file));

    let inital_code = fs::read_to_string(&inital_path).expect("Input-Datei fehlt");
    let expected_code = fs::read_to_string(&expected_path).expect("Expected-Datei fehlt");

    assert_ne!(inital_code, expected_code, "Dateien('{}' & '{}') sind gleich. Dies darf nicht sein, damit ein Idempotenz test wirkvoll sein kann.", inital_path.display(), expected_path.display());

    let mut lexer = Lexer::new(&inital_code, inital_path.to_string_lossy().to_string());
    let mut ast = lexer.parse_single_value().expect("Parsing des Inputs schlug fehl");
    ast.flatten();
    let first_format = ast.format_nix(0);

    assert_eq!( first_format, expected_code, "Fehler beim Formatieren der Datei: {}", file);

    let mut lexer_idempotent = Lexer::new(&first_format, "idempotent.nix".to_string());
    let mut ast_idempotent = lexer_idempotent.parse_single_value().expect("Parsing des formatierten Codes schlug fehl");
    ast_idempotent.flatten();
    let second_format = ast_idempotent.format_nix(0);

    assert_eq!( first_format, second_format, "Formatter ist nicht idempotent für Test: {}!", file);
    assert_eq!( second_format, expected_code, "Formatter ist nicht idempotent für Test: {}!", file);
}

fn idempotenz_sort_assert(file: &str) {
    let mut inital_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    inital_path.push(format!("tests/fixtures/sort/initial/{}.nix", file));
    expected_path.push(format!("tests/fixtures/sort/expected/{}.nix", file));

    let inital_code = fs::read_to_string(&inital_path).expect("Input-Datei fehlt");
    let expected_code = fs::read_to_string(&expected_path).expect("Expected-Datei fehlt");

    assert_ne!(inital_code, expected_code, "Dateien('{}' & '{}') sind gleich. Dies darf nicht sein, damit ein Idempotenz test wirkvoll sein kann.", inital_path.display(), expected_path.display());

    let mut lexer = Lexer::new(&inital_code, inital_path.to_string_lossy().to_string());
    let mut ast = lexer.parse_single_value().expect("Parsing des Inputs schlug fehl");
    ast.sort_ast();
    let first_format = ast.format_nix(0);

    assert_eq!( first_format, expected_code, "Fehler beim Formatieren der Datei: {}", file);

    let mut lexer_idempotent = Lexer::new(&first_format, "idempotent.nix".to_string());
    let mut ast_idempotent = lexer_idempotent.parse_single_value().expect("Parsing des formatierten Codes schlug fehl");
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


#[test]
fn int_idempotenz_expand_flake() {
    idempotenz_expand_assert("flake");
}

#[test]
#[ignore]
fn int_idempotenz_expand_github_runner() {
    idempotenz_expand_assert("github-runner");
}

#[test]
fn int_idempotenz_expand_hardware_configuration() {
    idempotenz_expand_assert("hardware-configuration");
}

#[test]
fn int_idempotenz_expand_matrix_server() {
    idempotenz_expand_assert("matrix-server");
}

#[test]
fn int_idempotenz_expand_monitoring() {
    idempotenz_expand_assert("monitoring");
}

#[test]
fn int_idempotenz_expand_shells() {
    idempotenz_expand_assert("shells");
}

#[test]
fn int_idempotenz_expand_tailscale() {
    idempotenz_expand_assert("tailscale");
}


#[test]
fn int_idempotenz_flatten_flake() {
    idempotenz_flatten_assert("flake");
}

#[test]
fn int_idempotenz_flatten_github_runner() {
    idempotenz_flatten_assert("github-runner");
}

#[test]
fn int_idempotenz_flatten_hardware_configuration() {
    idempotenz_flatten_assert("hardware-configuration");
}

#[test]
fn int_idempotenz_flatten_matrix_server() {
    idempotenz_flatten_assert("matrix-server");
}

#[test]
fn int_idempotenz_flatten_monitoring() {
    idempotenz_flatten_assert("monitoring");
}

#[test]
fn int_idempotenz_flatten_shells() {
    idempotenz_flatten_assert("shells");
}

#[test]
fn int_idempotenz_flatten_tailscale() {
    idempotenz_flatten_assert("tailscale");
}


#[test]
fn int_idempotenz_sort_flake() {
    idempotenz_sort_assert("flake");
}

#[test]
#[ignore]
fn int_idempotenz_sort_github_runner() {
    idempotenz_sort_assert("github-runner");
}

#[test]
fn int_idempotenz_sort_hardware_configuration() {
    idempotenz_sort_assert("hardware-configuration");
}

#[test]
fn int_idempotenz_sort_matrix_server() {
    idempotenz_sort_assert("matrix-server");
}

#[test]
#[ignore]
fn int_idempotenz_sort_monitoring() {
    idempotenz_sort_assert("monitoring");
}

#[test]
#[ignore]
fn int_idempotenz_sort_shells() {
    idempotenz_sort_assert("shells");
}

#[test]
fn int_idempotenz_sort_tailscale() {
    idempotenz_sort_assert("tailscale");
}
