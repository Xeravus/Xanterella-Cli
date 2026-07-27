use crate::config::xanterella::host::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_xanterella_host_init() {
        let data = XanterellaHostManager::init("path.nix");

        assert!(data.hosts.is_empty());
        assert_eq!(data.injection_path, String::from("path.nix"));
    }
}
