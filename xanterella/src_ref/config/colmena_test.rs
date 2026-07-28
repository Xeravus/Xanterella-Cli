use crate::config::colmena::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dummy_host(name: &str) -> ColmenaHost {
        ColmenaHost::new(name, "127.0.0.1", false, vec![])
    }

    #[test]
    fn test_colmena_add_host_with_builder() {
        let host = ColmenaHost::new(
            "crylia", 
            "192.168.1.50", 
            true, 
            vec!["./hardware-configuration.nix".to_string(), "./nginx.nix".to_string()]
        );
        
        let result = &host.finished_string;
        assert!(result.contains("crylia = {"));
        assert!(result.contains("deployment = {"));
        assert!(result.contains("targetHost = null;"));
        assert!(result.contains("allowLocalDeployment = true;"));
        assert!(result.contains("buildOnTarget = true;"));
        assert!(!result.contains("192.168.1.50"));
        assert!(result.contains("imports = ["));
        assert!(result.contains("./hardware-configuration.nix"));
        assert!(result.contains("./nginx.nix"));
        assert!(result.contains("];"));
        assert!(result.contains("};"));
    }

    #[test]
    fn test_colmena_add_host_without_builder() {
        let host = ColmenaHost::new("todesstern", "10.0.0.99", false, vec![]);
        
        let result = &host.finished_string;
        assert!(result.contains("todesstern = {"));
        assert!(result.contains("deployment = {"));
        assert!(result.contains("targetHost = \"10.0.0.99\";"));
        assert!(result.contains("buildOnTarget = false;"));
        assert!(result.contains("keys = commonSSHKeys;"));
        assert!(result.contains("imports = ["));
        assert!(!result.contains("targetHost = null"));
        assert!(result.contains("];"));
        assert!(result.contains("};"));
    }

    #[test]
    fn test_colmena_sort() {
        let mut file = ColmenaManager::init("/dummy/path");
        file.hosts = vec![
            create_dummy_host("zeta"),
            create_dummy_host("alpha"),
            create_dummy_host("omega"),
            create_dummy_host("beta"),
        ];
        
        file.sort_hosts(); 
        
        assert_eq!(file.hosts.len(), 4);
        assert_eq!(file.hosts[0].name, "alpha");
        assert_eq!(file.hosts[1].name, "beta");
        assert_eq!(file.hosts[2].name, "omega");
        assert_eq!(file.hosts[3].name, "zeta");
    }

    #[test]
    fn test_colmena_sort_already() {
        let mut file = ColmenaManager::init("/dummy/path");
        file.hosts = vec![create_dummy_host("crylia"), create_dummy_host("todesstern")];
        
        file.sort_hosts();
        
        assert_eq!(file.hosts[0].name, "crylia");
        assert_eq!(file.hosts[1].name, "todesstern");
    }

    #[test]
    fn test_colmena_sort_case_sens() {
        let mut file = ColmenaManager::init("/dummy/path");
        file.hosts = vec![create_dummy_host("Zeta"), create_dummy_host("alpha")];
        file.sort_hosts();
        assert_eq!(file.hosts[0].name, "Zeta"); 
        assert_eq!(file.hosts[1].name, "alpha");
    }

    #[test]
    fn test_colmena_gen_finished_string() {
        let mut file = ColmenaManager::init("/dummy/path");
        file.hosts = vec![create_dummy_host("alpha_node"), create_dummy_host("beta_node")];
        
        let result = file.gen_finished_string();
        
        assert!(result.contains("alpha_node = {"));
        assert!(result.contains("beta_node = {"));
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_colmena_replace_content() {
        let mut file = ColmenaManager::init("/dummy/path");
        file.content = "Dies ist ein Test.\n# --- Xanterella Hosts Start ---\nAlter Inhalt\n# --- Xanterella Hosts End ---\nEnde.".to_string();
        file.hosts = vec![create_dummy_host("new_node")];
        
        let result = file.content_replace();
        
        assert!(result.contains("Dies ist ein Test."));
        assert!(result.contains("new_node = {"));
        assert!(!result.contains("Alter Inhalt")); 
        assert!(result.contains("Ende."));
    }
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

    const MOCK_XERAVUS_BLOCK: &str = r#"  xeravus = {
    deployment = {
      targetHost = null;
      allowLocalDeployment = true;
      buildOnTarget = true;
    };
    imports = [
      ./hosts/xeravus/configuration.nix
    ];
  };"#;

    #[test]
    fn test_colmena_add_host_from_content1() {
        let host = ColmenaHost::new_from_content(MOCK_VICUNA_BLOCK);
        
        assert_eq!(host.name, "vicuna");
        assert_eq!(host.ip, "192.168.178.30");
        assert_eq!(host.builder, false);
        assert_eq!(host.imports.len(), 3);
        assert_eq!(host.imports[0], "./hosts/vicuna/configuration.nix");
        assert_eq!(host.imports[2], "inputs.nixos-hardware.nixosModules.raspberry-pi-5");
    }

    #[test]
    fn test_colmena_add_host_from_content2() {
        let host = ColmenaHost::new_from_content(MOCK_XERAVUS_BLOCK);
        
        assert_eq!(host.name, "xeravus");
        assert_eq!(host.ip, "null"); 
        assert_eq!(host.builder, true);
    }

    #[test]
    fn test_colmena_file_parse_hosts() {
        let mut file = ColmenaManager::init("/dummy/path");
        file.content = MOCK_COLMENA_FILE.to_string();
        
        file.parse_hosts();
        for i in &file.hosts {
            println!("{:#?}", i);
        }
        
        assert_eq!(file.hosts.len(), 2);
        
        assert_eq!(file.hosts[0].name, "xeravus");
        assert_eq!(file.hosts[0].builder, true);
        
        assert_eq!(file.hosts[1].name, "vicuna");
        assert_eq!(file.hosts[1].ip, "192.168.178.30");
    }
}
