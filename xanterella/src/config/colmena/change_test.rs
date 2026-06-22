use crate::config::colmena::change::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_host_config_with_remotebuilder() {
        let host = ColmenaHost {
            name: String::from("crylia"),
            ip: String::from("192.168.1.50"),
            remotebuilder: true,
            imports: vec![
                String::from("./hardware-configuration.nix"),
                String::from("./nginx.nix"),
            ],
        };
        let result = write_host_config(host);
        println!("{}", result);
        assert!(result.contains("crylia = {"));
        assert!(result.contains("targetHost = null;"));
        assert!(result.contains("allowLocalDeployment = true;"));
        assert!(result.contains("buildOnTarget = true;"));
        assert!(!result.contains("192.168.1.50"));
        assert!(result.contains("imports = [\n./hardware-configuration.nix\n./nginx.nix\n];"));
    }

    #[test]
    fn test_write_host_config_without_remotebuilder() {
        let host = ColmenaHost {
            name: String::from("todesstern"),
            ip: String::from("10.0.0.99"),
            remotebuilder: false,
            imports: vec![
                String::from("./hardware.nix"),
            ],
        };
        let result = write_host_config(host);
        assert!(result.contains("todesstern = {"));
        assert!(result.contains("targetHost = \"10.0.0.99\";"));
        assert!(result.contains("buildOnTarget = false;"));
        assert!(result.contains("keys = commonSSHKeys;"));
        assert!(!result.contains("targetHost = null"));
        assert!(!result.contains("allowLocalDeployment"));
    }
    fn create_dummy_host(name: &str) -> ColmenaHost {
        ColmenaHost {
            name: name.to_string(),
            ip: "127.0.0.1".to_string(),
            remotebuilder: false,
            imports: vec![],
        }
    }

    #[test]
    fn test_sort_hosts_alphabetically() {
        let file = ColmenaFile {
            hosts: vec![
                create_dummy_host("zeta"),
                create_dummy_host("alpha"),
                create_dummy_host("omega"),
                create_dummy_host("beta"),
            ],
        };
        let sorted_file = sort_hosts(file);
        assert_eq!(sorted_file.hosts.len(), 4);
        assert_eq!(sorted_file.hosts[0].name, "alpha");
        assert_eq!(sorted_file.hosts[1].name, "beta");
        assert_eq!(sorted_file.hosts[2].name, "omega");
        assert_eq!(sorted_file.hosts[3].name, "zeta");
    }

    #[test]
    fn test_sort_hosts_already_sorted() {
        let file = ColmenaFile {
            hosts: vec![
                create_dummy_host("crylia"),
                create_dummy_host("todesstern"),
            ],
        };
        let sorted_file = sort_hosts(file);
        assert_eq!(sorted_file.hosts[0].name, "crylia");
        assert_eq!(sorted_file.hosts[1].name, "todesstern");
    }

    #[test]
    fn test_sort_hosts_empty_list() {
        let file = ColmenaFile {
            hosts: vec![],
        };
        let sorted_file = sort_hosts(file);
        assert_eq!(sorted_file.hosts.len(), 0);
    }

    #[test]
    fn test_write_hosts_config_multiple_hosts() {
        let file = ColmenaFile {
            hosts: vec![
                create_dummy_host("alpha_node"),
                create_dummy_host("beta_node"),
            ],
        };
        let result = write_hosts_config(file);
        assert!(result.contains("alpha_node = {"));
        assert!(result.contains("beta_node = {"));
        assert!(result.contains('\n'));
    }

    #[test]
    fn test_write_hosts_config_empty_file() {
        let file = ColmenaFile {
            hosts: vec![],
        };
        let result = write_hosts_config(file);
        assert_eq!(result, "");
    }
    #[test]
    fn test_write_host_config_no_imports() {
        let host = ColmenaHost {
            name: String::from("ghost_node"),
            ip: String::from("192.168.1.100"),
            remotebuilder: false,
            imports: vec![], // Leer!
        };
        let result = write_host_config(host);
        assert!(result.contains("ghost_node = {"));
        assert!(result.contains("imports = [\n\n];")); 
    }
    #[test]
    fn test_sort_hosts_case_sensitivity() {
        let file = ColmenaFile {
            hosts: vec![
                create_dummy_host("Zeta"),
                create_dummy_host("alpha"),
            ],
        };
        let sorted_file = sort_hosts(file);
        assert_eq!(sorted_file.hosts[0].name, "Zeta");
        assert_eq!(sorted_file.hosts[1].name, "alpha");
    }
    #[test]
    fn test_generate_colmena_content() {
        let fake_content = "Dies ist ein Test.\n# --- Xanterella Hosts Start ---\nAlter Inhalt\n# --- Xanterella Hosts End ---";
        let file = ColmenaFile {
            hosts: vec![create_dummy_host("new_node")],
        };
        let result = generate_colmena_content(fake_content, file);
        assert!(result.contains("Dies ist ein Test."));
        assert!(result.contains("new_node = {"));
    }
}
