use std::fs;

use tempfile::{Builder, NamedTempFile};
use xanterella::config::xanterella::host::*;

#[test]
fn test_config_xanterella_host_load() {
    let temp = NamedTempFile::new().unwrap();
    let temp_path = temp.path().to_str().unwrap();

    fs::write(temp_path, "fake colmena config").unwrap();
}
