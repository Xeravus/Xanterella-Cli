use crate::install::ping::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_installer_ping_ping() {
        let xanterella1 = Xanterella::new();
        let xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.ping();
        let result2 = install2.ping();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::Ping(_))));
    }

    #[test]
    fn test_installer_ping_ping_ssh() {
        let xanterella1 = Xanterella::new();
        let xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.ping_ssh();
        let result2 = install2.ping_ssh();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::PingSsh(_))));
    }
}
