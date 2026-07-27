use crate::prelude::*;

pub struct Xanterella {
    ip: String,
    path: String,
    home: String,
    fast: bool,
    debug: bool,
    automate: bool,
    drive: Option<String>,
}

impl Xanterella {
    pub fn new() -> Self {
        Xanterella {
            ip: String::new(),
            path: String::new(),
            home: String::new(),
            fast: false,
            debug: false,
            automate: false,
            drive: None
        }
    }

    pub fn set_ip(&mut self, value: &str) {
        self.ip = value.to_string();
    }

    pub fn set_path(&mut self, value: &str) {
        self.path = value.to_string();
    }

    pub fn set_home(&mut self, value: &str) {
        self.home = value.to_string();
    }

    pub fn set_fast(&mut self, value: bool) {
        self.fast = value;
    }

    pub fn set_debug(&mut self, value: bool) {
        self.debug = value;
    }

    pub fn set_automate(&mut self, value: bool) {
        self.automate = value;
    }

    pub fn set_drive(&mut self, value: &str) {
        self.drive = Some(value.to_string());
    }

    pub fn log_event(&mut self, event: Events, value: Option<Cow<'static, str>>) {
        let extra: Cow<'static, str> = match value {
            Some(val) => val.into(),
            None => "".into(),
        };

        let (run, name): (bool, Cow<'static, str>) = match event {
            Events::RunPing => (true, "Starte Ping"),
            Events::OkPing => (false, "Ping erfolgreich"),

        };
        
        let prefix = match run {
            true => "[ RUN ] ",
            false => "[ OK ] ",
        };

        println!("{}", (format!("{}{}", prefix, name)));
    }
}
