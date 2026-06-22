use crate::installer::file::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_edit_config_add_hardware() {
        let dummy_config = String::from("\n{\n  imports = [\n    ./module.nix\n  ];\n}\n");
        let result = edit_config(dummy_config, EditMode::Add);

        assert!(result.contains("./hardware-configuration.nix"));
        assert!(result.contains("imports = ["));
    }

    #[test]
    fn test_edit_config_remove_hardware() {
        let dummy_config = String::from("\n  imports = [\n    ./hardware-configuration.nix\n    ./module.nix\n  ];\n");
        let result = edit_config(dummy_config, EditMode::Remove);

        assert!(!result.contains("./hardware-configuration.nix"));
        assert!(result.contains("./module.nix"));
    }
}
