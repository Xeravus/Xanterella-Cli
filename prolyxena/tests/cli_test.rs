use std::env::set_var;
use std::fs;

use prolyxena::cli::commands::*;
use tempfile::Builder;

#[test]
fn int_test_cli_commands_prolyxena_parse_no_stdout() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, false, false, false, false, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, false, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.is_empty());
}

#[test]
fn int_test_cli_commands_prolyxena_parse_time() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, false, false, false, true, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, true, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.contains("Time: "));
}

#[test]
fn int_test_cli_commands_prolyxena_parse_output() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, false, false, true, false, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_parse(&mut buffer, temp_path.to_string(), false, true, false, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.contains("File"));
}

#[test]
fn int_test_cli_commands_prolyxena_parse_all() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, false, false, true, true, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_parse(&mut buffer, temp_path.to_string(), false, true, true, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(!output_string.is_empty());
    assert!(output_string.contains("Time: "));
    assert!(output_string.contains("Dir("));
}

#[test]
fn int_test_cli_commands_prolyxena_parse_animation() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    unsafe {
        set_var("PROLYXENA_TEST", "1");
    }
    prolyxena_parse(&mut buffer, temp_path.to_string(), false, false, false, true, false, false, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    unsafe {
        set_var("PROLYXENA_TEST", "1");
    }

    prolyxena_parse(&mut buffer, temp_path.to_string(), true, false, false, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.is_empty());
}

#[test]
fn int_test_cli_commands_prolyxena_format_no_stdout() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, false, false, false, false, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, false, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.is_empty());
}

#[test]
fn int_test_cli_commands_prolyxena_format_time() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, false, false, false, true, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, true, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.contains("Time: "));
}

#[test]
fn int_test_cli_commands_prolyxena_format_output() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, false, false, true, false, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_format(&mut buffer, temp_path.to_string(), false, true, false, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.contains("File"));
}

#[test]
fn int_test_cli_commands_prolyxena_format_all() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, false, false, true, true, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    prolyxena_format(&mut buffer, temp_path.to_string(), false, true, true, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(!output_string.is_empty());
    assert!(output_string.contains("Time: "));
    assert!(output_string.contains("Dir("));
}

#[test]
fn int_test_cli_commands_prolyxena_format_animation() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");
    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();
    let mut buffer = Vec::new();
    unsafe {
        set_var("PROLYXENA_TEST", "1");
    }
    prolyxena_format(&mut buffer, temp_path.to_string(), false, false, false, true, false, false, false);

    let temp_file = Builder::new().suffix(".nix").tempfile().unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut buffer = Vec::new();

    unsafe {
        set_var("PROLYXENA_TEST", "1");
    }

    prolyxena_format(&mut buffer, temp_path.to_string(), true, false, false, false);

    let output_string = String::from_utf8(buffer).unwrap();

    assert!(output_string.is_empty());
}
