use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Data {
    pub tailkey: String,
    pub wifi: String,
}

pub trait Config {
    fn config_create_dir(&self) -> Result<(), EventsFailed>;
    fn config_gen_basic(&self) -> Result<(), EventsFailed>;
    fn config_parse(&self) -> Result<Data, EventsFailed>;
}

impl Config for Xanterella {
    fn config_create_dir(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigCreateDir);

        fs::create_dir_all(self.get_path(Paths::Config))
            .map_err(|err| EventsFailed::CreateDir(err.to_string()))?;

        self.log_event(Events::OkConfigCreateDir);
        Ok(())
    }

    fn config_gen_basic(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunConfigGenBasic);

        let basic = Data {
            tailkey: String::from("tskey-auth-XXXXXXXXXXXXXXXXX-YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"),
            wifi: String::from("Obi Wlan Kenobi"),
        };
        let json_string = serde_json::to_string_pretty(&basic).unwrap();
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        fs::write(&json_path, &json_string)
            .map_err(|err| EventsFailed::Fs(err.to_string()))?;

        self.log_event(Events::OkConfigGenBasic);
        Ok(())
    }

    fn config_parse(&self) -> Result<Data, EventsFailed> {
        let json_path = PathBuf::from(self.get_path(Paths::Config)).join("config.json").display().to_string();
        let file_content = fs::read_to_string(&json_path).map_err(|err| EventsFailed::Fs(err.to_string()))?;
        serde_json::from_str(&file_content)
            .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
    }
}
