use log::{error, info};

use std::process::{self, Command};

use crate::utils::get::*;

pub fn part_efi(drive: &str, debug: &bool, ip: &str) {
    info!("[ RUN ] - Erstelle Partition EFI");

    if !debug {
        let parted_efi = Command::new("ssh")
            .args(get_sshstring(ip, User::Root))
            .arg("parted")
            .arg("-s")
            .arg(drive)
            .args(["mklabel", "gpt"])
            .args(["mkpart", "disk-main-boot", "fat32", "1Mib", "512MiB"])
            .args(["set", "1", "esp", "on"])
            .output()
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte 'parted' nicht starten: {}", err);
                process::exit(1);
            });
        if !parted_efi.status.success() {
            error!(
                "[ FAILED ] - Konnte das Drive nicht partitionieren: {}",
                String::from_utf8_lossy(&parted_efi.stderr)
            );
            process::exit(1);
        }
    };
    info!("[ OK ] - Partition EFI erstellt");
}

pub fn part_root(drive: &str, debug: &bool, ip: &str) {
    info!("[ RUN ] - Erstelle Partition ROOT");

    if !debug {
        let parted_root = Command::new("ssh")
            .args(get_sshstring(ip, User::Root))
            .arg("parted")
            .arg("-s")
            .arg(drive)
            .args(["mkpart", "disk-main-root", "ext4", "512MiB", "100%"])
            .output()
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte parted nicht starten: {}", err);
                process::exit(1);
            });
        if !parted_root.status.success() {
            error!(
                "[ FAILED ] - Konnte das Drive nicht partitionieren: {}",
                String::from_utf8_lossy(&parted_root.stderr)
            );
            process::exit(1);
        }
    };
    info!("[ OK ] - Partition Root erstellt");
}

pub fn format_efi(primdrive: &str, debug: &bool, ip: &str) {
    info!("[ RUN ] - Starte Formatierung von EFI");
    if !debug {
        let mkfs_efi = Command::new("ssh")
            .args(get_sshstring(ip, User::Root))
            .arg("mkfs.fat")
            .arg(get_drives_name(primdrive, 1))
            .args(["-F", "32", "-n", "boot"])
            .output()
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err);
                process::exit(1);
            });
        if !mkfs_efi.status.success() {
            error!(
                "[ FAILED ] - Konnte die Partition nicht formatieren: {}",
                String::from_utf8_lossy(&mkfs_efi.stderr)
            );
            process::exit(1);
        }
    };
    info!("[ OK ] - Formatierung von EFI erfolgreich");
}

pub fn format_root(primdrive: &str, debug: &bool, ip: &str) {
    info!("[ RUN ] - Starte Formatierung von ROOT");
    if !debug {
        let mkfs_root = Command::new("ssh")
            .args(get_sshstring(ip, User::Root))
            .arg("mkfs.ext4")
            .arg(get_drives_name(primdrive, 2))
            .args(["-L", "nixos"])
            .output()
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte Mkfs.ext4 nicht starten: {}", err);
                process::exit(1);
            });
        if !mkfs_root.status.success() {
            error!(
                "[ FAILED ] - Konnte die Partition nicht formatieren: {}",
                String::from_utf8_lossy(&mkfs_root.stderr)
            );
            process::exit(1);
        }
    };
    info!("[ OK ] - Formatierung von ROOT erfolgreich");
}

pub fn mount_root(primdrive: &str, ip: &str) {
    info!("[ RUN ] - Starte Mounting von root");

    let root = Command::new("ssh")
        .args(get_sshstring(ip, User::Root))
        .arg("mount")
        .arg(get_drives_name(primdrive, 2))
        .arg("/mnt")
        .output()
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte mount nicht starten: {}", err);
            process::exit(1);
        });
    if !root.status.success() {
        error!("[ FAILED ] - Konnte die Root Partition nicht mounten: {}", String::from_utf8_lossy(&root.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Mounting von root erfolgreich");
}

pub fn create_boot_dir(ip: &str) {
    info!("[ OK ] - Starte Erstellung des Boot Dir");

    let dir = Command::new("ssh")
        .args(get_sshstring(ip, User::Root))
        .arg("mkdir")
        .arg("-p")
        .arg("/mnt/boot")
        .output()
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte mkdir nicht starten: {}", err);
            process::exit(1);
        });
    if !dir.status.success() {
        error!("[ FAILED ] - Konnte die den Boot Ordner nicht erstellen: {}", String::from_utf8_lossy(&dir.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Erstellung des Boot Dir erfolgreich");
}

pub fn mount_boot(primdrive: &str, ip: &str) {
    info!("[ RUN ] - Starte Mounting von root");

    let boot = Command::new("ssh")
        .args(get_sshstring(ip, User::Root))
        .arg("mount")
        .arg(get_drives_name(primdrive, 1))
        .arg("/mnt/boot")
        .output()
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte mount nicht starten: {}", err);
            process::exit(1);
        });
    if !boot.status.success() {
        error!("[ FAILED ] - Konnte die Boot Partition nicht mounten: {}", String::from_utf8_lossy(&boot.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Mounting von boot erfolgreich");
}
