use crate::XanterellaInstall;
use crate::install::helper::Helper;
use prolyxena::engine::lexer::vfs::FsData;
use prolyxena::engine::generator::generate::Modify;
use prolyxena::engine::formater::write::Write;

use crate::prelude::*;

pub trait Edit {
    fn crylia_edit_start(&mut self) -> Result<(), EventsFailed>;
}

impl<'a> Edit for XanterellaInstall<'a> {
    fn crylia_edit_start(&mut self) -> Result<(), EventsFailed> {
        let mut config = FsData::new(&self.xanterella.path);
        config.load().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        let hardware = self.get_hardware()?;
        config.generate_file("/hosts/crylia/hardware-configuration.nix", hardware).map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        config.walk_tree().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        Ok(())
    }
}
