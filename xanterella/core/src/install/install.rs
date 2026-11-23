use crate::xanterella::CommandRunner;
use crate::Events;
use crate::Xanterella;


pub struct XanterellaInstall<C: CommandRunner> {
    pub xanterella: Xanterella<C>,
    pub ip: String,
    pub drive: String,
}

impl<C: CommandRunner> XanterellaInstall<C> {
    #[allow(clippy::new_without_default)]
    pub fn new(xanterella: Xanterella<C>) -> Self {
        XanterellaInstall {
            xanterella,
            ip: String::new(),
            drive: String::new(),
        }
    }

    pub fn set_ip(&mut self, value: &str) {
        self.ip = value.to_string();
    }

    pub fn set_drive(&mut self, value: &str) {
        self.drive = value.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_install_debug() -> XanterellaInstall {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);
        install.xanterella.debug = true;
        install
    }

    #[test]
    fn test_installer_core_new() {
        let xanterella = Xanterella::new();
        let install = XanterellaInstall::new(xanterella);

        assert!(matches!(
            install,
            XanterellaInstall {
                xanterella: _,
                ip: _,
                drive: _,
            }
        ));
        assert!(matches!(
            install.xanterella,
            Xanterella {
                config: _,
                fast: _,
                debug: _,
                automate: _,
                sender: _,
            }
        ));
        assert!(install.ip.is_empty());
        assert!(install.drive.is_empty());
    }

    #[test]
    fn test_installer_core_set_ip() {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);

        install.set_ip("127.0.0.1");
        assert!(!install.ip.is_empty());
        assert_eq!(install.ip, String::from("127.0.0.1"));
    }

    #[test]
    fn test_installer_core_set_drive() {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);

        install.set_drive("nvme");
        assert!(!install.drive.is_empty());
        assert_eq!(install.drive, String::from("nvme"));
    }
}
