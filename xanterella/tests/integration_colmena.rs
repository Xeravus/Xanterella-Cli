use tempfile::{NamedTempFile, Builder};
use std::fs;

use xanterella::config::colmena::parse::*;
use xanterella::config::colmena::change::*;
use xanterella::config::core::*;

#[test]
fn test_direct_nested_function() {
    let temp = NamedTempFile::new().unwrap();
    let fake_path = temp.path().to_str().unwrap();

    fs::write(fake_path, "fake colmena config").unwrap();
    let content = colmena_get_content(fake_path); 
    assert_eq!(content, "fake colmena config");
}

#[test]
fn test_integration_add_host_with_fixtures1() {
    let initial_content = include_str!("fixtures/colmena/initial.nix");
    let expected_content = include_str!("fixtures/colmena/added.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_add_host(temp_path, "prolyxena", "1.1.1.1", false);
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_add_host_with_fixtures2() {
    let initial_content = include_str!("fixtures/colmena/removed.nix");
    let expected_content = include_str!("fixtures/colmena/initial.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_add_host(temp_path, "vicuna", "192.168.178.30", false);
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_remove_host_with_fixtures_with_name1() {
    let initial_content = include_str!("fixtures/colmena/initial.nix");
    let expected_content = include_str!("fixtures/colmena/removed.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_remove_host(temp_path, Some("vicuna"), None);
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_remove_host_with_fixtures_with_name2() {
    let initial_content = include_str!("fixtures/colmena/added.nix");
    let expected_content = include_str!("fixtures/colmena/initial.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_remove_host(temp_path, Some("prolyxena"), None);
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_remove_host_with_fixtures_with_ip1() {
    let initial_content = include_str!("fixtures/colmena/initial.nix");
    let expected_content = include_str!("fixtures/colmena/removed.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_remove_host(temp_path, None, Some("192.168.178.30"));
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_remove_host_with_fixtures_with_ip2() {
    let initial_content = include_str!("fixtures/colmena/added.nix");
    let expected_content = include_str!("fixtures/colmena/initial.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_remove_host(temp_path, None, Some("1.1.1.1"));
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_remove_host_with_fixtures_with_name_and_ip1() {
    let initial_content = include_str!("fixtures/colmena/initial.nix");
    let expected_content = include_str!("fixtures/colmena/removed.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_remove_host(temp_path, None, Some("192.168.178.30"));
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_remove_host_with_fixtures_with_name_and_ip2() {
    let initial_content = include_str!("fixtures/colmena/added.nix");
    let expected_content = include_str!("fixtures/colmena/initial.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    write_remove_host(temp_path, None, Some("1.1.1.1"));
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_sort_hosts1() {
    let initial_content = include_str!("fixtures/colmena/unsorted.nix");
    let expected_content = include_str!("fixtures/colmena/initial.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    rewrite_hosts(temp_path);
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}

#[test]
fn test_integration_sort_hosts2() {
    let initial_content = include_str!("fixtures/colmena/unsorted.nix");
    let expected_content = include_str!("fixtures/colmena/sorted.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();
    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, initial_content).unwrap();

    rewrite_hosts(temp_path);
    let result_content = fs::read_to_string(temp_path).unwrap();

    assert_eq!(result_content, expected_content);
}
