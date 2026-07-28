use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Data {
    pub tailkey: String,
    pub wifi: String,
}

pub trait Config {
    fn config_create_dir(&mut self) -> Result<(), EventsFailed>;
    fn config_gen_basic(&mut self) -> Result<(), EventsFailed>;
    fn config_parse(&mut self) -> Result<Data, EventsFailed>;
}

impl Config for Xanterella {
    fn config_create_dir(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigCreateDir);

        fs::create_dir_all(self.get_path(Paths::Config))
            .map_err(|err| EventsFailed::ConfigCreateDir(err.to_string()))?;

        self.log_event(Events::OkConfigCreateDir);
        Ok(())
    }

    fn config_gen_basic(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigGenBasic);

        let basic = Data {
            tailkey: String::from("tskey-auth-XXXXXXXXXXXXXXXXX-YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"),
            wifi: String::from("Obi Wlan Kenobi"),
        };
        let json_string = serde_json::to_string_pretty(&basic).unwrap();
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        fs::write(&json_path, &json_string).map_err(|err| EventsFailed::Fs(err.to_string()))?;

        self.log_event(Events::OkConfigGenBasic);
        Ok(())
    }

    fn config_parse(&mut self) -> Result<Data, EventsFailed> {
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        let file_content = fs::read_to_string(&json_path).map_err(|err| EventsFailed::Fs(err.to_string()))?;
        Ok(serde_json::from_str::<Data>(&file_content).map_err(|err| EventsFailed::SerdeJson(err.to_string()))?)
    }
}
