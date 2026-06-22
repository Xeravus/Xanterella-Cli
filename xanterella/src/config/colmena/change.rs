use log::{info, error};

use crate::config::colmena::parse::*;

/*
pub fn write_hosts(file: ColmenaFile) {
}
*/

pub fn sort_hosts(hosts: ColmenaFile) -> ColmenaFile {
    let mut output: Vec<ColmenaHost> = hosts.hosts;
    output.sort_by(|a, b| a.name.cmp(&b.name));
    let data = ColmenaFile {
        hosts: output,
    };
    data
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
}};",
    host.name, deployment_block, imports_block
    )
}

#[cfg(test)]
#[path = "change_test.rs"]
mod tests;
