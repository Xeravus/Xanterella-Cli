use std::process;

use cliclack::*;
use prolyxena::engine::lexer::vfs::*;
use tokio::sync::broadcast;
use xanterella_core::{
    Git, Xanterella, XanterellaInstall,
    get::Get,
    install::drives::Drives,
    prolyxena::Nixtractor,
    xanterella::{EventFormat, EventState},
};

pub async fn execute_remote_install(automate: bool, speed: bool, debug: bool, flake: &str) {
    if intro("Start Installer").is_err() {
        println!("CliClack could not initialize");
        process::exit(2);
    };
    let init_spinner = spinner();

    let (tx, mut rx) = broadcast::channel::<EventFormat>(100);

    init_spinner.start("Init");
    let mut xanterella = Xanterella::new();
    if let Err(err) = xanterella.set_flake_path(flake) {
        println!("Error: {:#?}", err);
        process::exit(1);
    };
    let mut installer = XanterellaInstall::new(xanterella.clone());
    installer.xanterella.set_automate(automate);
    installer.xanterella.set_fast(speed);
    installer.xanterella.set_debug(debug);
    installer.xanterella.set_sender(tx);
    init_spinner.stop("Init success");

    let ip = choose_device(&xanterella).await;
    let drive = choose_drive(&mut installer).await;

    installer.set_ip(&ip);
    installer.set_drive(&drive);

    let bar = progress_bar(30);
    bar.start("Installer");

    tokio::spawn(async move {
        if let Err(e) = installer.remote_integration() {
            println!("Install-Error: \nStage: 'Remote Integration' \n{:#?}", e);
            if let Err(e) = installer.xanterella.git_rollback(1) {
                println!("Rollback-Error: \nGit could rollback. \n{:#?} \nCritical Error", e);
                process::exit(2);
            }
        }
        if let Err(e) = installer.remote_prep_fs() {
            println!("Install-Error: \nStage: 'Remote Prep Filesystem' \n{:#?}", e);
            process::exit(1);
        }
        if let Err(e) = installer.remote_install() {
            println!("Install-Error: \nStage: 'Remote Install' \n{:#?}", e);
            process::exit(1);
        }
        if let Err(e) = installer.remote_install_cleanup() {
            println!("Install-Error: \nStage: 'Remote Install Cleanup' \n{:#?}", e);
            if let Err(e) = installer.xanterella.git_rollback(1) {
                println!("Rollback-Error: \nGit could rollback. \n{:#?} \nCritical Error", e);
                process::exit(2);
            }
        }
    });
    let mut spin = spinner();
    while let Ok(msg) = rx.recv().await {
        match msg.state {
            EventState::Run => {
                spin.start(format!("Start {}", msg.step));
            }
            EventState::Finish => {
                spin.stop(format!("Finished {}", msg.step));
                spin = spinner();
                bar.inc(1);
            }
            EventState::Failed => {
                spin.error(format!("Abort {}", msg.step));
                break;
            }
        }
    }

    bar.stop("Installation complete");

    if outro("Finished Installer").is_err() {
        println!("CliClack could end. \nDont worry the installer is finished");
        process::exit(1);
    }
}

async fn choose_device(xanterella: &Xanterella) -> String {
    if let Ok(devices) = xanterella.get_taildevices() {
        let mut select = select("Choose the target");
        for (_, i) in devices.devices {
            let ip = i.ip[0].clone();
            select = select.item(ip.clone(), format!("{:>15} - {}", i.name, ip), "");
        }
        if let Ok(ans) = select.interact() { ans.to_string() } else { String::from("127.0.0.1") }
    } else {
        String::from("127.0.0.1")
    }
}

async fn choose_drive(xanterella: &mut XanterellaInstall) -> String {
    if let Ok(drives) = xanterella.get_drives() {
        let mut select = select("Choose the main Drive");
        for i in drives.blockdevices {
            select = select.item(i.name.clone(), format!("{:>20} - {}", i.name, i.size), "");
        }
        if let Ok(ans) = select.interact() { ans.to_string() } else { String::from("/dev/null") }
    } else {
        String::from("/dev/null")
    }
}

pub async fn execute_extract(flake: &str) {
    let mut prolyxena1 = FsData::new(flake);
    let mut prolyxena2 = FsData::new(flake);
    let mut prolyxena3 = FsData::new(flake);
    let mut prolyxena4 = FsData::new(flake);
    if let Err(e) = prolyxena1.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    if let Err(e) = prolyxena2.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    if let Err(e) = prolyxena3.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    if let Err(e) = prolyxena4.load() {
        eprintln!("Excration-Error: '{}'", e)
    }
    let extractor1 = Nixtractor::new(&mut prolyxena1);
    let extractor2 = Nixtractor::new(&mut prolyxena2);
    let extractor3 = Nixtractor::new(&mut prolyxena3);
    let extractor4 = Nixtractor::new(&mut prolyxena4);
    match extractor1.await.extract_hosts().await {
        Ok(_) => println!("Hosts OK"),
        Err(err) => panic!("Error while extracting the Hosts: \n '{}'", err),
    };
    match extractor2.await.extract_profiles().await {
        Ok(_) => println!("Profiles OK"),
        Err(err) => panic!("Error while extracting the Profiles: \n '{}'", err),
    };
    match extractor3.await.extract_modules().await {
        Ok(_) => println!("Modules OK"),
        Err(err) => panic!("Error while extracting the Modules: \n '{}'", err),
    };
    match extractor4.await.extract_templates().await {
        Ok(_) => println!("Templates OK"),
        Err(err) => panic!("Error while extracting the Templates: \n '{}'", err),
    };
}
