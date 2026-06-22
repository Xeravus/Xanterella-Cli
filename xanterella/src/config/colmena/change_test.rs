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
}
