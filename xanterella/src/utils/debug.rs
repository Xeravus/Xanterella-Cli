use clap::ValueEnum;

use crate::init::core::*;
use crate::installer::core::*;
use crate::installer::install::*;
use crate::usb::core::*;
use crate::usb::flash::*;
use crate::utils::get::*;
use crate::utils::select::*;
use crate::config::colmena::parse::*;

#[derive(ValueEnum, Clone, Debug)]
pub enum ListDebug {
    Reboot,
    Drives,
    Taildevices,
    Select,
    Iso,
    Flash,
    Hardware,
    Init,
    Colmena,
}

pub fn list_debug(function: &ListDebug) {
    match function {
        ListDebug::Reboot => {
            reboot("127.0.0.1", &true);
        }
        ListDebug::Drives => {
            let drives = get_drives("127.0.0.1");
            println!("{:?}", drives);
            for i in &drives.blockdevices {
                println!(" - - - - - - -");
                println!("{}", i.name);
                println!("  {}", i.size);
                println!("  {}", i.device_type);
            }
            drives_part(&select_drive("127.0.0.1", &true), &true, "127.0.0.1");
            drives_part(&select_drive("127.0.0.1", &false), &true, "127.0.0.1");
        }
        ListDebug::Taildevices => {
            println!("{:?}", get_taildevices());
        }
        ListDebug::Select => {
            println!("{:?}", get_taildevices());
            println!("{:?}", get_drives("127.0.0.1"));
        }
        ListDebug::Iso => {
            println!("{}", build_iso(&true));
        }
        ListDebug::Flash => {
            flash_usb(true);
        }
        ListDebug::Hardware => {
            let target_ip = select_host(get_taildevices());
            println!("{}", get_hardware(&target_ip));
        }
        ListDebug::Init => {
            init_git("127.0.0.1");
        }
        ListDebug::Colmena => {
            let colmena = colmena_parse_hosts(&get_path(Paths::Colmena));
            println!("{:#?}", colmena);
            for i in &colmena.hosts {
                println!(" - - - - - - - - - - ");
                println!("Name: {}", i.name);
                println!("IP: {}", i.ip);
                println!("RemoteBuilder: {}", i.remotebuilder);
                println!("Imports: {:#?}", i.imports);
            }
        }
    }
}
