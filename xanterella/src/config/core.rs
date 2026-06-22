use log::{info};

use crate::config::query::*;
use crate::config::filepaths::*;
use crate::config::colmena::change::*;
use crate::config::colmena::parse::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", convert_filepath(&i, OutPath::Last, false));
    }
}

pub fn rewrite_hosts() {
    write_hosts(&colmena_get_content(), sort_hosts(colmena_parse_hosts()))
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
