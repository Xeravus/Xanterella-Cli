use crate::XanterellaInstall;
use prolyxena::engine::lexer::vfs::FsData;

pub trait Edit {
    fn crylia_edit_start(&mut self, hardware: String) -> Result<(), String>;
}

impl<'a> Edit for XanterellaInstall<'a> {
    fn crylia_edit_start(&mut self, hardware: String) -> Result<(), String> {
        let mut config = FsData::new(&self.xanterella.path);
        config.load();
        Ok(())
    }
}
