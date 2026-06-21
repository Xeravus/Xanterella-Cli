use crate::config::colmena::parse::*;

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_COLMENA_FILE: &str = r#"
  };
  # --- Xanterella Hosts Start ---
  xeravus = {
    deployment = {
      targetHost = null;
      allowLocalDeployment = true;
      buildOnTarget = true;
    };
    imports = [
      ./hosts/xeravus/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };
  vicuna = {
    deployment = {
      targetHost = "192.168.178.30";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
      inputs.nixos-hardware.nixosModules.raspberry-pi-5
    ];
  };
  # --- Xanterella Hosts End ---
}
"#;

    const MOCK_VICUNA_BLOCK: &str = r#"  vicuna = {
    deployment = {
      targetHost = "192.168.178.30";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
      inputs.nixos-hardware.nixosModules.raspberry-pi-5
    ];
  };"#;

    #[test]
    fn test_colmena_split_marker() {
        let extracted = colmena_split_marker(MOCK_COLMENA_FILE);
        
        assert!(!extracted.contains("meta = {"));
        assert!(!extracted.contains("# --- Xanterella Hosts Start ---"));
        
        assert!(extracted.contains("xeravus = {"));
        assert!(extracted.contains("vicuna = {"));
    }

    #[test]
    fn test_colmena_extract_name() {
        let name = colmena_extract_name(MOCK_VICUNA_BLOCK);
        assert_eq!(name.trim(), "vicuna");
    }

    #[test]
    fn test_colmena_extract_ip() {
        let ip = colmena_extract_ip(MOCK_VICUNA_BLOCK);
        assert_eq!(ip.trim(), "192.168.178.30");
    }

    #[test]
    fn test_colmena_extract_ip_null() {
        let xeravus_block = r#"
        xeravus = {
            deployment = {
              targetHost = null;
              allowLocalDeployment = true;
              buildOnTarget = true;
            };"#;
        let ip = colmena_extract_ip(xeravus_block);
        assert_eq!(ip.trim(), "null");
    }

    #[test]
    fn test_colmena_extract_remotebuilder() {
        let is_remote = colmena_extract_remotebuilder(MOCK_VICUNA_BLOCK);
        assert_eq!(is_remote, false); // Vicuna hat false
        
        let mock_lutik = "buildOnTarget = true;";
        assert_eq!(colmena_extract_remotebuilder(mock_lutik), true); // Lutik hat true
    }

    #[test]
    fn test_colmena_extract_imports() {
        let imports = colmena_extract_imports(MOCK_VICUNA_BLOCK);
        
        assert_eq!(imports.len(), 3);
        assert_eq!(imports[0], "./hosts/vicuna/configuration.nix");
        assert_eq!(imports[1], "./profiles/ssh-keys.nix");
        assert_eq!(imports[2], "inputs.nixos-hardware.nixosModules.raspberry-pi-5");
    }
}
