use crate::{Events, Xanterella, xanterella::CommandRunner};

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
