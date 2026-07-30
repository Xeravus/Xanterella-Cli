use crate::install::inject::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installer_inject_inject_tailscale() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.inject_tailscale();
        let result2 = install2.inject_tailscale();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(_)));
    }

    #[test]
    fn test_installer_inject_inject_wifi() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.inject_wifi();
        let result2 = install2.inject_wifi();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(_)));
    }
}
