use clap::ValueEnum;

use crate::init::core::*;
use crate::installer::core::*;
use crate::installer::install::*;
use crate::usb::core::*;
use crate::usb::flash::*;
use crate::utils::get::*;
use crate::utils::select::*;

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
    }
}
