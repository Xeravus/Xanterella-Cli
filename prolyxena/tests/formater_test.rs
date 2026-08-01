use tempfile::{Builder, NamedTempFile};
use std::fs;

use prolyxena::engine::core::*;
use prolyxena::engine::lexer::core::*;
use prolyxena::engine::lexer::vfs::*;

#[test]
fn int_test_formater_colmena_host_nix() {
    let inital_content = include_str!("fixtures/colmena-hosts.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut data_nrm = FsData::new(&temp_path);
    let mut data_fmt = FsData::new(&temp_path);

    data_nrm.sort(false);
    data_fmt.sort(true);

    data_nrm.load();
    data_fmt.load();

    let db_nrm = format!("{:#?}", data_nrm.fsnodes);
    let db_fmt = format!("{:#?}", data_fmt.fsnodes);

    assert_ne!(db_nrm, db_fmt);
    assert_ne!(data_nrm.sort, data_fmt.sort);
    assert_eq!(data_nrm.files, data_fmt.files);
    assert_eq!(data_nrm.path, data_fmt.path);
}

#[test]
fn int_test_formater_modules_boot_boot_nix() {
    let inital_content = include_str!("fixtures/boot.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut data_nrm = FsData::new(&temp_path);
    let mut data_fmt = FsData::new(&temp_path);

    data_nrm.sort(false);
    data_fmt.sort(true);

    data_nrm.load();
    data_fmt.load();

    let db_nrm = format!("{:#?}", data_nrm.fsnodes);
    let db_fmt = format!("{:#?}", data_fmt.fsnodes);

    assert_ne!(db_nrm, db_fmt);
    assert_ne!(data_nrm.sort, data_fmt.sort);
    assert_eq!(data_nrm.files, data_fmt.files);
    assert_eq!(data_nrm.path, data_fmt.path);
}

#[test]
fn int_test_formater_modules_essentials_tailscale_nix() {
    let inital_content = include_str!("fixtures/tailscale.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut data_nrm = FsData::new(&temp_path);
    let mut data_fmt = FsData::new(&temp_path);

    data_nrm.sort(false);
    data_fmt.sort(true);

    data_nrm.load();
    data_fmt.load();

    let db_nrm = format!("{:#?}", data_nrm.fsnodes);
    let db_fmt = format!("{:#?}", data_fmt.fsnodes);

    assert_ne!(db_nrm, db_fmt);
    assert_ne!(data_nrm.sort, data_fmt.sort);
    assert_eq!(data_nrm.files, data_fmt.files);
    assert_eq!(data_nrm.path, data_fmt.path);
}

#[test]
fn int_test_formater_let_in() {
    let inital_content = include_str!("fixtures/fmt/let-in.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut data_nrm = FsData::new(&temp_path);
    let mut data_fmt = FsData::new(&temp_path);

    data_nrm.sort(false);
    data_fmt.sort(true);

    data_nrm.load();
    data_fmt.load();

    let db_nrm = format!("{:#?}", data_nrm.fsnodes);
    let db_fmt = format!("{:#?}", data_fmt.fsnodes);

    assert_ne!(db_nrm, db_fmt);
    assert_ne!(data_nrm.sort, data_fmt.sort);
    assert_eq!(data_nrm.files, data_fmt.files);
    assert_eq!(data_nrm.path, data_fmt.path);
}

#[test]
fn int_test_formater_with() {
    let inital_content = include_str!("fixtures/fmt/with.nix");

    let temp_file = Builder::new()
        .suffix(".nix")
        .tempfile()
        .unwrap();

    let temp_path = temp_file.path().to_str().unwrap();
    fs::write(temp_path, inital_content).unwrap();

    let mut data_nrm = FsData::new(&temp_path);
    let mut data_fmt = FsData::new(&temp_path);

    data_nrm.sort(false);
    data_fmt.sort(true);

    data_nrm.load();
    data_fmt.load();

    let db_nrm = format!("{:#?}", data_nrm.fsnodes);
    let db_fmt = format!("{:#?}", data_fmt.fsnodes);

    assert_ne!(db_nrm, db_fmt);
    assert_ne!(data_nrm.sort, data_fmt.sort);
    assert_eq!(data_nrm.files, data_fmt.files);
    assert_eq!(data_nrm.path, data_fmt.path);
}
