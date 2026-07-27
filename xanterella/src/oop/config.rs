#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tailkey: String,
    pub wifi: String,
}

pub trait Config {
    fn config_create_dir(&self) -> Result<(), EventsFailed>;
    fn config_gen_basic(&self) -> Result<(), EventsFailed>;
}

impl Config for Xanterella {
    fn config_create_dir(&self) -> Result<(), EventsFailed> {
    }

    fn config_gen_basic(&self) -> Result<(), EventsFailed> {
    }
}
