use prolyxena::engine::formater::write::Write;
use prolyxena::engine::generator::generate::Generate;
use prolyxena::engine::generator::generate::Modify;
use prolyxena::engine::generator::query::Search;
use prolyxena::engine::generator::remove::Delete;
use prolyxena::engine::generator::remove::Remove;
use prolyxena::engine::lexer::vfs::FsData;

use crate::XanterellaInstall;
use crate::install::helper::Helper;
use crate::XanterellaInstall;
use crate::install::helper::Helper;
use prolyxena::engine::lexer::vfs::FsData;
use prolyxena::engine::generator::generate::Modify;
use prolyxena::engine::formater::write::Write;

use crate::prelude::*;

pub trait Edit {
    fn crylia_edit_start(&mut self) -> Result<(), EventsFailed>;
    fn crylia_add_hardware_conf(&mut self, config: &mut FsData) -> Result<(), EventsFailed>;
    fn crylia_add_hardware_link(&mut self, config: &mut FsData) -> Result<(), EventsFailed>;

    fn crylia_edit_finish(&mut self) -> Result<(), EventsFailed>;
    fn crylia_remove_hardware_conf(&mut self, config: &mut FsData) -> Result<(), EventsFailed>;
    fn crylia_remove_hardware_link(&mut self, config: &mut FsData) -> Result<(), EventsFailed>;
}

impl<'a> Edit for XanterellaInstall<'a> {
    fn crylia_edit_start(&mut self) -> Result<(), EventsFailed> {
        let mut config = FsData::new(&self.xanterella.path);
        self.crylia_add_hardware_conf(&mut config)?;
        self.crylia_add_hardware_link(&mut config)?;
        Ok(())
    }
    fn crylia_add_hardware_conf(&mut self, config: &mut FsData) -> Result<(), EventsFailed> {
        if !self.xanterella.debug {
            config.load().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
            let hardware = self.get_hardware()?;
            config
                .generate_file("/hosts/crylia/hardware-configuration.nix", hardware)
                .map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
            config.walk_tree().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        }
        Ok(())
    }

    fn crylia_add_hardware_link(&mut self, config: &mut FsData) -> Result<(), EventsFailed> {
        if !self.xanterella.debug {
            let file = config
                .search_tree("hosts/crylia/configuration.nix")
                .map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
            file.insert("imports", "./hardware-configuration.nix")
                .map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        }
        Ok(())
    }

    fn crylia_edit_finish(&mut self) -> Result<(), EventsFailed> {
        let mut config = FsData::new(&self.xanterella.path);
        self.crylia_remove_hardware_conf(&mut config)?;
        self.crylia_remove_hardware_link(&mut config)?;
        Ok(())
    }
    fn crylia_remove_hardware_conf(&mut self, config: &mut FsData) -> Result<(), EventsFailed> {
        if !self.xanterella.debug {
            config.load().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
            config
                .delete_file("/hosts/crylia/hardware-configuration.nix")
                .map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        }
        Ok(())
    }

    fn crylia_remove_hardware_link(&mut self, config: &mut FsData) -> Result<(), EventsFailed> {
        if !self.xanterella.debug {
            let file = config
                .search_tree("hosts/crylia/configuration.nix")
                .map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
            file.remove("imports", "./hardware-configuration.nix")
                .map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        }
        config.load().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        let hardware = self.get_hardware()?;
        config.generate_file("/hosts/crylia/hardware-configuration.nix", hardware).map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        config.walk_tree().map_err(|err| EventsFailed::Prolyxena(err.to_string()))?;
        Ok(())
    }
}
