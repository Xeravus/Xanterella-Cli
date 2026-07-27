use inquire::Select;
use log::{debug, error};
use strum::IntoEnumIterator;

use std::fmt::Display;
use std::process;

use crate::utils::get::*;

pub fn select_host(hosts: Taildevices) -> String {
    let mut options: Vec<String> = vec![];
    let mut output_ip: String = String::from("127.0.0.1");
    for (_pubkey, device_info) in hosts.devices {
        let ip: &str = device_info.ip.first().map(|s| s.as_str()).unwrap_or("Keine IP");
        let input = format!("IP: {:<15} - OS: {:<7} Name: {}", ip, device_info.os, device_info.name);
        options.push(input);
    }
    let answer = Select::new("Select Hosts", options).prompt();
    match answer {
        Ok(choice) => {
            if let Some((ip, _name)) = choice.split_once(" - OS: ") {
                let clean_ip: &str = ip.strip_prefix("IP: ").unwrap_or(ip).trim();
                output_ip = String::from(clean_ip);
            }
        }
        Err(e) => {
            error!("[ FAILED ] - Konnte den Input nicht auslesen: {}", e);
            process::exit(1);
        }
    }
    debug!("Output IP: {}", output_ip);
    output_ip
}

pub fn select_drive(target_ip: &str, automate: &bool) -> String {
    let mut options: Vec<String> = vec![];
    let drives = get_sort_drives(get_drives(target_ip)).blockdevices;
    for i in &drives {
        let input = format!("Name: {:<7} - Size: {}", i.name, i.size);
        options.push(input);
    }
    if !automate {
        let answer = Select::new("Select Disk", options).prompt();
        let mut output_name: String = String::from("");
        match answer {
            Ok(choice) => {
                if let Some((name, _size)) = choice.split_once(" - Size: ") {
                    let clean_name: &str = name.strip_prefix("Name: ").unwrap_or(name).trim();
                    output_name = String::from(clean_name);
                }
            }
            Err(e) => {
                error!("[ FAILED ] - Konnte den Input nicht auslesen: {}", e);
                process::exit(1);
            }
        }
        debug!("Output Name: {}", output_name);
        output_name
    } else {
        drives[0].name.clone()
    }
}

pub fn select_mode<T>(msg: &str) -> T
where
    T: IntoEnumIterator + Display + Clone,
{
    let options: Vec<T> = T::iter().collect();
    let ans = Select::new(msg, options).prompt();
    ans.unwrap_or_else(|err| {
        error!("[ FAILED - Konnte den Input nicht auslesen: {}", err);
        process::exit(1);
    })
}
