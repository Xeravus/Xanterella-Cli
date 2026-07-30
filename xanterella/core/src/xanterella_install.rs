use crate::prelude::*;

pub struct XanterellaInstall<'a> {
    pub xanterella: &'a mut Xanterella,
    pub ip: String,
    pub drive: String,
}

impl<'a> XanterellaInstall<'a> {
    pub fn new(xanterella: &'a mut Xanterella) -> Self {
        XanterellaInstall {
            xanterella,
            ip: String::new(),
            drive: String::new(),
        }
    }

    pub fn get_sshstring(&mut self, user: User) -> Vec<String> {
        let target = match user {
            User::Root => format!("root@{}", self.ip),
            User::Cato => format!("cato@{}", self.ip),
        };
        vec![
            "-o".to_string(),
            "StrictHostKeyChecking=no".to_string(),
            "-o".to_string(),
            "UserKnownHostsFile=/dev/null".to_string(),
            target,
        ]
    }

}
