use crate::prelude::*;

use prolyxena::*;
use prolyxena::engine::lexer::vfs::*;
use serde::*;
use serde_json::Value;

pub struct Nixtractor {
    pub prolyxena: FsData,
}

#[derive(Deserialize)]
pub struct CreateHost {
    pub hostname: String,
    pub ip: String,
    pub profiles: Vec<Value>,
    pub options: Vec<Value>,
}


impl Nixtractor {
    pub async fn extract_hosts(&self) -> Result<Vec<CreateHost>, String> {
        unimplemented!();
    }
}
