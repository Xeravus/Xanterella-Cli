use log::{info};

use crate::config::query::*;
use crate::config::filepaths::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", convert_filepath(&i, OutPath::Last, false));
    }
}

pub fn list_modules() {
    info!("Modules: ");
    for i in query_modules_all() {
        info!("{}", convert_filepath(&i, OutPath::Shortend, false));
    }
}

pub fn list_profiles() {
    info!("Profiles: ");
    for i in query_profiles() {
        info!("{}", convert_filepath(&i, OutPath::Shortend, false));
    }
}
