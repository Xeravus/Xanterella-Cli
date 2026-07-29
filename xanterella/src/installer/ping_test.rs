use crate::installer::ping::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn test_installer_ping_ping() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();
        let mut xanterella3 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);
        let mut install3 = XanterellaInstall::new(&mut xanterella3);

        install1.xanterella.debug = true;
        install2.ip = "127.0.0.1".to_string();
        install3.ip = "127.127.127.127.127".to_string();

        let result1 = install1.ping();
        let result2 = install2.ping();
        let result3 = install3.ping();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err());

        assert!(matches!(result3, Err(EventsFailed::Ping(_))));
    }

    #[test]
    fn test_installer_ping_ping_ssh() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.ping_ssh();
        let result2 = install2.ping_ssh();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::PingSsh(_))));
    }
}
