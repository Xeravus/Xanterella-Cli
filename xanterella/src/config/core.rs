use log::{info, debug};

use crate::config::query::*;
use crate::config::filepaths::*;
use crate::config::colmena::change::*;
use crate::config::colmena::parse::*;
use crate::config::colmena::hosts::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", convert_filepath(&i, OutPath::Last, false));
    }
}

pub fn write_add_host(name: &str, ip: &str, remotebuilder: bool) {
    write_hosts(&colmena_get_content(), sort_hosts(colmena_add_host(name, ip, remotebuilder)));
}

pub fn rewrite_hosts() {
    debug!("rewrite_hosts(content): \n{:#?}\n - - - - - - - - - - - - - ", colmena_get_content());
    debug!("rewrite_hosts(parsed hosts): \n{:#?}\n - - - - - - - - - - - - - - - - - ", colmena_parse_hosts());
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
