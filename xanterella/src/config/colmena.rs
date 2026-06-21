use crate::utils::get::*;

pub struct ColmenaFile {
    hosts: Vec<ColmenaHost>,
}

pub struct ColmenaHost {
    name: String,
    ip: String,
    remotebuilder: bool,
    imports: Vec<String>,
}

/*
pub fn parse_colmena_hosts() -> ColmenaFile {
    let content = fs::read_to_string(get_path(Paths::Colmena)).expect("[ FAILED ] - Konnte die Colmena Host Datei nicht auslesen");
}
*/

pub fn colmena_split_marker(content: String) -> String {
    let (_start, hosts_start) = content
        .split_once("# --- Xanterella Hosts Start ---")
        .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts Start ---)");
    let (host_final, _end) = hosts_start
        .split_once("# --- Xanterella Hosts End ---")
        .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts End ---)");
    host_final.to_string()
}

pub fn colmena_split_hosts(content: String) -> Vec<String> {
    let teile: Vec<&str> = content.split("
    ];
  };
  ").collect();
    let mut output: Vec<String> = vec![];
    for i in teile {
        output.push(format!("{} ]; }}; ", i))
    }
    output
}

pub fn colmena_extract_name(content: String) -> String {
    let (name, _rest) = content.split_once(" = {").unwrap();
    name.to_string()
}

pub fn colmena_extract_ip(content: String) -> String {
    let (_rest, ip_teil) = content.split_once("targetHost = ").unwrap();
    let (ip, _rest) = ip_teil.split_once(";").unwrap();
    ip.replace('"', "")
}

pub fn colmena_extract_remotebuilder(content: String) -> bool {
    let (_rest, remote_teil) = content.split_once("buildOnTarget = ").unwrap();
    let (remote, _rest) = remote_teil.split_once(";").unwrap();
    let remote_builder = remote.trim().parse::<bool>().unwrap();
    remote_builder
}

pub fn colmena_extract_imports(content: String) -> Vec<String> {
    let (_rest, remote_teil) = content.split_once("imports = [").unwrap();
    let (remote, _rest) = remote_teil.split_once("];").unwrap();
    let mut output: Vec<String> = vec![];
    for i in remote.trim().lines() {
        output.push(i.trim().to_string())
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // Wir simulieren exakt den Aufbau deiner colmena-hosts.nix Datei
    const MOCK_COLMENA_FILE: &str = r#"
  };
  # --- Xanterella Hosts Start ---
  xeravus = {
    deployment = {
      targetHost = null;
      allowLocalDeployment = true;
      buildOnTarget = true;
    };
    imports = [
      ./hosts/xeravus/configuration.nix
      ./profiles/ssh-keys.nix
    ];
  };
  vicuna = {
    deployment = {
      targetHost = "192.168.178.30";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
      inputs.nixos-hardware.nixosModules.raspberry-pi-5
    ];
  };
  # --- Xanterella Hosts End ---
}
"#;

    const MOCK_VICUNA_BLOCK: &str = r#"  vicuna = {
    deployment = {
      targetHost = "192.168.178.30";
      targetUser = taruser;
      buildOnTarget = false;
      keys = commonSSHKeys;
    };
    imports = [
      ./hosts/vicuna/configuration.nix
      ./profiles/ssh-keys.nix
      inputs.nixos-hardware.nixosModules.raspberry-pi-5
    ];
  };"#;

    #[test]
    fn test_colmena_split_marker() {
        let extracted = colmena_split_marker(MOCK_COLMENA_FILE.to_string());
        
        // Darf nicht mehr den Meta-Block oder das Dateiende enthalten
        assert!(!extracted.contains("meta = {"));
        assert!(!extracted.contains("# --- Xanterella Hosts Start ---"));
        
        // Muss die Hosts enthalten
        assert!(extracted.contains("xeravus = {"));
        assert!(extracted.contains("vicuna = {"));
    }

    #[test]
    fn test_colmena_extract_name() {
        let name = colmena_extract_name(MOCK_VICUNA_BLOCK.to_string());
        // Erwartet, dass vor dem " = {" sauber abgeschnitten wird
        assert_eq!(name.trim(), "vicuna");
    }

    #[test]
    fn test_colmena_extract_ip() {
        let ip = colmena_extract_ip(MOCK_VICUNA_BLOCK.to_string());
        // Darf keine Anführungszeichen oder Semikolons mehr enthalten
        assert_eq!(ip.trim(), "192.168.178.30");
    }

    #[test]
    fn test_colmena_extract_ip_null() {
        // Testet den Sonderfall xeravus (targetHost = null;)
        let xeravus_block = r#"
        xeravus = {
            deployment = {
              targetHost = null;
              allowLocalDeployment = true;
              buildOnTarget = true;
            };"#;
        let ip = colmena_extract_ip(xeravus_block.to_string());
        assert_eq!(ip.trim(), "null");
    }

    #[test]
    fn test_colmena_extract_remotebuilder() {
        let is_remote = colmena_extract_remotebuilder(MOCK_VICUNA_BLOCK.to_string());
        assert_eq!(is_remote, false); // Vicuna hat false
        
        let mock_lutik = "buildOnTarget = true;";
        assert_eq!(colmena_extract_remotebuilder(mock_lutik.to_string()), true); // Lutik hat true
    }

    #[test]
    fn test_colmena_extract_imports() {
        let imports = colmena_extract_imports(MOCK_VICUNA_BLOCK.to_string());
        
        assert_eq!(imports.len(), 3);
        assert_eq!(imports[0], "./hosts/vicuna/configuration.nix");
        assert_eq!(imports[1], "./profiles/ssh-keys.nix");
        assert_eq!(imports[2], "inputs.nixos-hardware.nixosModules.raspberry-pi-5");
    }
}
