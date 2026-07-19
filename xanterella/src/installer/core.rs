use log::info;

use crate::init::core::*;
use crate::installer::drives::*;
use crate::installer::file::*;
use crate::installer::install::*;
use crate::utils::check::*;
use crate::utils::core::*;
use crate::utils::get::*;
use crate::utils::git::git_full;
use crate::utils::select::*;

pub fn remote_install(automate: &bool, fast: &bool, debug: &bool) {
    let target_ip = select_host(get_taildevices());
    ping_full(&target_ip);
    crylia_edit_start(get_hardware(&target_ip));
    git_full(String::from("Xanterella Remote-Install"));
    if !*fast {
        nix_check();
    };
    let primdrive = select_drive(&target_ip, automate);
    drives_part(&primdrive, debug, &target_ip);
    drives_format(&primdrive, debug, &target_ip);
    drives_mount(&primdrive, &target_ip);
    build(debug);
    deploy(&target_ip, fast, debug);
    reboot(&target_ip, debug);
    // -----------------------------------------------------
    clean();
}

pub fn daemon_install(automate: bool, fast: bool, ip: String, debug: bool) {
    let target_ip: &str = &ip;
    ping_full(target_ip);
    crylia_edit_start(get_hardware(target_ip));
    git_full(String::from("Xanterella Remote-Install"));
    if !fast {
        nix_check();
    };
    let primdrive = select_drive(target_ip, &automate);
    drives_part(&primdrive, &debug, target_ip);
    drives_format(&primdrive, &debug, target_ip);
    drives_mount(&primdrive, target_ip);
    build(&debug);
    deploy(target_ip, &fast, &debug);
    reboot(target_ip, &false);
    // -----------------------------------------------------
    clean();
}

pub fn clean() {
    info!("[ RUN ] - Starte Cleanup");

    crylia_edit_finish();
    git_full(String::from("Xanterella Remote-Install cleanup"));
    info!("[ OK ] - Cleanup erfolgreich");
}

pub fn crylia_edit_start(config: String) {
    info!("[ RUN ] - Starte Crylia Überarbeitung");

    create_hardware(config);
    write_config(edit_config(parse_config(), EditMode::Add));
    files_alejandra(&get_path(Paths::Nixconf));
    info!("[ OK ] - Crylia Überarbeitung erfolgreich");
}

pub fn crylia_edit_finish() {
    info!("[ RUN ] - Starte Crylia Überarbeitung");

    remove_hardware();
    write_config(edit_config(parse_config(), EditMode::Remove));
    files_alejandra(&get_path(Paths::Nixconf));
    info!("[ OK ] - Crylia Überarbeitung erfolgreich");
}

pub fn drives_part(primdrive: &str, debug: &bool, ip: &str) {
    info!("[ RUN ] - Starte Parittionierung");

    let drive = format!("/dev/{}", primdrive);
    part_efi(&drive, debug, ip);
    part_root(&drive, debug, ip);
    info!("[ OK ] - Parittionierung erfolgreich");
}

pub fn drives_format(primdrive: &str, debug: &bool, ip: &str) {
    info!("[ RUN ] - Starte Formatierung");

    format_efi(primdrive, debug, ip);
    format_root(primdrive, debug, ip);
    info!("[ Ok ] - Formatierung erfolgreich");
}

pub fn drives_mount(primdrive: &str, ip: &str) {
    info!("[ RUN ] - Starte Mounting");

    mount_root(primdrive, ip);
    create_boot_dir(ip);
    mount_boot(primdrive, ip);
    info!("[ OK ] - Mounting erfolgreich");
}

pub fn deploy(ip: &str, fast: &bool, debug: &bool) {
    info!("[ RUN ] - Starte Deployment");

    if !debug {
        copy(ip, fast);
        profile(ip);
        prep(ip);
        activate(ip);
        inject(ip);
        bootloader(ip);
    }
    info!("[ OK ] - Deployment erfolgreich");
}
