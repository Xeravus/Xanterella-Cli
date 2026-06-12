use log::{info, error, debug};

use std::process::{self, Command};
use std::fs;
use std::time::Instant;
use std::time::Duration;

use crate::utils::get::*;

pub fn build(debug: &bool) {
    info!("[ RUN ] - Starte lokalen Build");
    
    if !debug {
        let build = Command::new("nix")
            .args(["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"])
            .current_dir(get_path(Paths::Nixconf))
            .output()
            .unwrap_or_else(|err| { 
                error!("Konnte nix build nicht starten: {}", err); 
                process::exit(1); 
            });
        if !build.status.success() {
            error!("[ FAILED ] - Lokaler Build fehlgeschlagen: {}", String::from_utf8_lossy(&build.stderr));
            process::exit(1);
        }
    }
    info!("[ OK ] - lokaler Build erfolgreich");
}

pub fn copy(ip: &str, fast: &bool) {
    info!("[ RUN ] - Starte Copy des Closure");

    let start = Instant::now();
    if !fast {
        let fast_cmd = format!("nix-store --export $(nix.store -qR ./result) | zstd -T0 -3 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -C root@{} 'zstdcat | nix-store --store /mnt --import'", ip);
        let copy = Command::new("sh")
            .arg("-c")
            .arg(fast_cmd)
            .current_dir(get_path(Paths::Nixconf))
            .output()
            .unwrap_or_else(|err| { 
                error!("Konnte nix copy nicht starten: {}", err); 
                process::exit(1); 
            });
        if !copy.status.success() {
            let err = String::from_utf8_lossy(&copy.stderr);
            error!("[ FAILED ] - Kopieren der System-Closure fehlgeschlagen:\n{}", err);
            process::exit(1);
        }
    } else {
        let copy = Command::new("nix")
            .env("NIX_SSHOPTS", "-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -C")
            .args([
                "copy", 
                "--no-check-sigs",
                "--substitute-on-destination", 
                "--to", 
                &format!("ssh-ng://root@{}?remote-store=local%3Froot%3D/mnt", ip), 
                "./result"
            ])
            .current_dir(get_path(Paths::Nixconf))
            .output()
            .unwrap_or_else(|err| { 
                error!("Konnte nix copy nicht starten: {}", err); 
                process::exit(1); 
            });
        if !copy.status.success() {
            let err = String::from_utf8_lossy(&copy.stderr);
            error!("[ FAILED ] - Kopieren der System-Closure fehlgeschlagen:\n{}", err);
            process::exit(1);
        }
    }
    info!("[ OK ] - Copy des Closure erfolgreich");
    info!("[ TIME ] Copy des Closure: {:?}", start.elapsed());
}

pub fn profile(ip: &str) {
    info!("[ RUN ] - Starte Aktivierung des Profiles");

    let system_path = fs::read_link(format!("{}/result", get_path(Paths::Nixconf)))
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte Symlink 'result' nicht auflösen: {}", err); 
            process::exit(1); 
        })
        .to_string_lossy()
        .into_owned();
    debug!("System-Pfad im Nix-Store: {}", system_path);
    let profile_cmd = format!("nix-env --store /mnt -p /mnt/nix/var/nix/profiles/system --set {}", system_path);
    let profile = Command::new("ssh")
        .arg(get_sshstring(ip, User::Root))
        .arg(&profile_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });
    if !profile.status.success() {
        error!("[ FAILED ] - Profil-Registrierung fehlgeschlagen: {}", String::from_utf8_lossy(&profile.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Aktivierung des Profiles erfolgreich");
}

pub fn prep(ip: &str) {
    info!("[ RUN ] - Starte Vorbereitung des Dateisystem für nixos-enter vor");

    let prep_cmd = "mkdir -m 0755 -p /mnt/etc && touch /mnt/etc/NIXOS";
    let prep = Command::new("ssh")
        .arg(get_sshstring(ip, User::Root))
        .arg(prep_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });

    if !prep.status.success() {
        error!("[ FAILED ] - Vorbereitung fehlgeschlagen: {}", String::from_utf8_lossy(&prep.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Vorbereitung erfolgreich");
}

pub fn activate(ip: &str) {
    info!("[ RUN ] - Aktiviere das System");

    let activate_cmd = "NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'";
    let activate = Command::new("ssh")
        .arg(get_sshstring(ip, User::Root))
        .arg(activate_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });
    if !activate.status.success() {
        error!("[ FAILED ] - Systemaktivierung fehlgeschlagen: {}", String::from_utf8_lossy(&activate.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Aktivierung des Systems erfolgreich");
}

pub fn bootloader(ip: &str) {
    info!("[ RUN ] - Starte Aktualisiere Bootloader");

    let bootloader_cmd = "nixos-enter --root /mnt --command 'NIXOS_INSTALL_BOOTLOADER=1 /nix/var/nix/profiles/system/bin/switch-to-configuration boot'";
    let bootloader = Command::new("ssh")
        .arg(get_sshstring(ip, User::Root))
        .arg(bootloader_cmd)
        .output()
        .unwrap_or_else(|err| { error!("Konnte 'ssh' oder 'nix' nicht starten: {}", err); process::exit(1); });
    if !bootloader.status.success() {
        error!("[ FAILED ] - Bootloader Installation fehlgeschlagen: {}", String::from_utf8_lossy(&bootloader.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Aktualisierung des Bootloaders erfolgreich");
}

pub fn reboot(ip: &str, debug: &bool) {
    info!("[ RUN ] - Logge aus Tailscale aus und starte neu...");

    if !debug {
        let magic_cmd = "nohup sh -c 'sleep 2 && tailscale logout && reboot' > /dev/null 2>&1 &";
        let logout_reboot = Command::new("ssh")
            .arg(format!("root@{}", ip))
            .arg(magic_cmd)
            .output();
        match logout_reboot {
            Ok(output) => {
                if output.status.success() {
                    info!("[ OK ] - Befehl abgesetzt. Das Gerät loggt sich aus und startet neu.");
                } else {
                    error!("[ FAILED ] - SSH Befehl schlug fehl: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                error!("[ FAILED ] - Konnte SSH nicht ausführen: {}", e);
            }
        }
    }
}
