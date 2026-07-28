use crate::prelude::*;

pub struct Xanterella {
    pub path: String,
    pub home: String,
    pub fast: bool,
    pub debug: bool,
    pub automate: bool,
}

impl Xanterella {
    pub fn new() -> Self {
        Xanterella {
            path: String::new(),
            home: String::new(),
            fast: false,
            debug: false,
            automate: false,
        }
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

    pub fn log_event(&mut self, event: Events) {
        let (run, name): (bool, String) = match event {
            Events::RunPing => (true, "Starte Ping".into()),
            Events::OkPing => (false, "Ping erfolgreich".into()),
            _ => todo!(),
        };

        let prefix = match run {
            true => "[ RUN ] ",
            false => "[ OK ] ",
        };

        println!("{}", (format!("{}{}", prefix, name)));
    }
}
