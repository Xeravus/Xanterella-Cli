use log::{info, error};

use std::fs;

use crate::config::colmena::parse::*;
use crate::utils::get::*;

pub fn write_hosts(content: &str, file: ColmenaFile) {
    let replaced_content = content.replace(&colmena_split_marker(&content), &write_hosts_config(file));
    let _ = fs::write(get_path(Paths::Colmena), replaced_content);
}

pub fn sort_hosts(hosts: ColmenaFile) -> ColmenaFile {
    let mut output: Vec<ColmenaHost> = hosts.hosts;
    output.sort_by(|a, b| a.name.cmp(&b.name));
    ColmenaFile {
        hosts: output,
    }
}

pub fn write_hosts_config(file: ColmenaFile) -> String {
    let mut hosts: Vec<String> = vec![];
    for i in file.hosts {
        hosts.push(write_host_config(i))
    }
    hosts.join("\n")
}

pub fn write_host_config(host: ColmenaHost) -> String {
    let deployment_block = if host.remotebuilder {
        String::from("targetHost = null;\nallowLocalDeployment = true;\nbuildOnTarget = true;\n")
    } else {
        format!("targetHost = \"{}\";\nkeys = commonSSHKeys;\nbuildOnTarget = false;", host.ip)
    };

    let imports_block = host.imports.join("\n");

    format!("
{} = {{
deployment = {{
{}
}};
imports = [
{}
];
}};\n",
    host.name, deployment_block, imports_block
    )
}

#[cfg(test)]
#[path = "change_test.rs"]
mod tests;
