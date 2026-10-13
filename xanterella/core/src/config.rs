use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Data {
    pub tailkey: String,
    pub wifi: String,
    pub flake: String,
}

pub trait Config {
    fn config_create_dir(&mut self) -> Result<(), EventsFailed>;
    fn config_write(&mut self, data: Data) -> Result<(), EventsFailed>;
    fn config_gen_basic(&mut self) -> Result<(), EventsFailed>;
    fn config_parse(&mut self) -> Result<Data, EventsFailed>;
    fn config_set_tailkey(&mut self, value: String) -> Result<Data, EventsFailed>;
}

impl Config for Xanterella {
    fn config_create_dir(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigCreateDir);

        fs::create_dir_all(self.get_path(Paths::Config))
            .map_err(|err| EventsFailed::ConfigCreateDir(err.to_string()))?;

        self.log_event(Events::OkConfigCreateDir);
        Ok(())
    }

    fn config_write(&mut self, data: Data) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigWrite);

        let json_string =
            serde_json::to_string_pretty(&data).map_err(|err| EventsFailed::SerdeJson(err.to_string()))?;
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        fs::write(&json_path, &json_string).map_err(|err| EventsFailed::Fs(err.to_string()))?;

        self.log_event(Events::OkConfigWrite);
        Ok(())
    }

    fn config_gen_basic(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigGenBasic);

        let basic = Data {
            tailkey: String::from("tskey-auth-XXXXXXXXXXXXXXXXX-YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"),
            wifi: String::from("Obi Wlan Kenobi"),
            flake: String::from("flake/flake.nix"),
        };
        let json_string =
            serde_json::to_string_pretty(&basic).map_err(|err| EventsFailed::SerdeJson(err.to_string()))?;
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        fs::write(&json_path, &json_string).map_err(|err| EventsFailed::Fs(err.to_string()))?;

        self.log_event(Events::OkConfigGenBasic);
        Ok(())
    }

    fn config_parse(&mut self) -> Result<Data, EventsFailed> {
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        let file_content = fs::read_to_string(&json_path).map_err(|err| EventsFailed::Fs(err.to_string()))?;
        serde_json::from_str::<Data>(&file_content).map_err(|err| EventsFailed::SerdeJson(err.to_string()))
    }

    fn config_set_tailkey(&mut self, value: String) -> Result<Data, EventsFailed> {
        let input = self.config_parse()?;
        Ok(Data {
            tailkey: value,
            ..input
        })
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn test_config_lifecycle() {
        let mut x = Xanterella::new();
        // Temporären isolierten Ordner nutzen, um echte Configs nicht zu überschreiben
        let temp_dir = std::env::temp_dir()
            .join(format!("xant_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()));
        x.set_home(temp_dir.to_str().unwrap());

        // 1. Ordner erstellen
        assert!(x.config_create_dir().is_ok());
        assert!(temp_dir.join(".config").join("xanterella").exists());

        // 2. Standard-Config generieren
        assert!(x.config_gen_basic().is_ok());

        // 3. Config einlesen
        let parsed = x.config_parse().expect("Sollte parsen können");
        assert_eq!(parsed.flake, "flake/flake.nix");

        // 4. Config ändern und speichern
        let new_data = x.config_set_tailkey("test-key-123".to_string()).unwrap();
        assert!(x.config_write(new_data).is_ok());

        // 5. Änderung verifizieren
        let verified = x.config_parse().unwrap();
        assert_eq!(verified.tailkey, "test-key-123");

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
