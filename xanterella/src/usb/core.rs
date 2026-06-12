use log::info;

use crate::usb::flash::*;
use crate::utils::select::*;
use crate::utils::get::*;

pub fn flash_usb(debug: bool) {
    let mode: FlashMode = select_mode("In welchem Modus soll geflasht werden: ");
    info!("[ RUN ] - Starte Flashing");

    match mode {
        FlashMode::Local => {
            let ip = "127.0.0.1";
            flash_iso(&select_drive(ip, &false), &build_iso(&debug), &mode, ip, &debug);
        },
        FlashMode::Remote => {
            let ip = select_host(get_taildevices());
            if debug {
                flash_iso(&select_drive("127.0.0.1", &false), &build_iso(&debug), &mode, "127.0.0.1", &debug);
            } else {
                flash_iso(&select_drive(&ip, &false), &build_iso(&debug), &mode, &ip, &debug);
            }
        },
    }
    info!("[ OK ] - Flashing erfolgreich");
}
    
