use log::{debug, info, error};

use std::process::{self, Command};
use std::fs;

use crate::utils::get::*;

pub enum EditMode {
    Add,
    Remove,
}

pub fn create_hardware(config: String) {
    info!("[ RUN ] - Starte Erstellung der Hardware Config für Crylia");

    let file_path = format!("{}/hosts/crylia/hardware-configuration.nix", get_path(Paths::Nixconf));
    fs::write(&file_path, &config)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Hardware Config nicht schreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Erstellung der Hardware Config für Crylia erfolgreich");
}

pub fn remove_hardware() {
    info!("[ RUN ] - Starte Löschung von der Hardware Config von Crylia");

    let file_path = format!("{}/hosts/crylia/hardware-configuration.nix", get_path(Paths::Nixconf));
    fs::remove_file(file_path)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Hardware Config nicht löschen: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Löschung von der Hardware Config von Crylia erfolgreic erfolgreichh");
}

pub fn parse_config() -> String {
    info!("[ RUN ] - Starte Parse für den Inhalt von Crylia");

    let file_path = format!("{}/hosts/crylia/configuration.nix", get_path(Paths::Nixconf));
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht auslesen: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Parse erfolgreich");
    content
}

pub fn edit_config(content: String, mode: EditMode) -> String {
    info!("[ RUN ] - Starte Inhalt überarbeitung");

    let result = match mode {
        EditMode::Add => {
            let Some((anfang, ende)) = content.split_once("  imports = [") else {
                error!("[ FAILED ] - Konnte 'imports = [' nicht in der Config von Crylia finden");
                process::exit(1);
            };
            let whole_content = format!(
                "{}
                imports = [
                ./hardware-configuration.nix
                {}", anfang, ende);
            debug!("Neuer Inhalt: \n{}", whole_content);
            whole_content
        },
        EditMode::Remove => {
            let whole_content = content.replace("    ./hardware-configuration.nix\n", "");
            debug!("Neuer Inhalt: \n{}", whole_content);
            whole_content
        },
    };
    info!("[ OK ] - Inhalt überarbeitung erfolgreich");
    result
}

pub fn write_config(content: String) {
    info!("[ RUN ] - Starte Schreibprozess von der Hardware Config von Crylia");

    let file_path = format!("{}/hosts/crylia/configuration.nix", get_path(Paths::Nixconf));
    fs::write(file_path, content)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte die Config von Crylia nicht überschreiben: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Schreibprozess von der Hardware Config von Crylia erfolgreich");
}

pub fn files_alejandra() {
    info!("[ RUN ] - Starte Alejandra");

    let alejandra = Command::new("alejandra")
        .arg(".")
        .current_dir(get_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte Alejandra nicht starten: {}", err); 
            process::exit(1); 
        });
    debug!("Alejandra: \n{}", String::from_utf8_lossy(&alejandra.stdout));
    if !alejandra.status.success() {
        error!("[ FAILED ] - Konnte die Dateien mit Alejandra nicht formatieren: {}", String::from_utf8_lossy(&alejandra.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Alejandra erfolgreich");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_edit_config_add_hardware() {
        let dummy_config = String::from("\n{\n  imports = [\n    ./module.nix\n  ];\n}\n");
        let result = edit_config(dummy_config, EditMode::Add);
        
        assert!(result.contains("./hardware-configuration.nix"));
        assert!(result.contains("imports = [")); 
    }

    #[test]
    fn test_edit_config_remove_hardware() {
        let dummy_config = String::from("\n  imports = [\n    ./hardware-configuration.nix\n    ./module.nix\n  ];\n");
        let result = edit_config(dummy_config, EditMode::Remove);

        assert!(!result.contains("./hardware-configuration.nix"));
        assert!(result.contains("./module.nix"));
    }
}

