use log::{info, error};

pub fn write_hosts(file: ColmenaFile) {
}

pub fn sort_hosts(hosts: ColmenaFile) -> ColmenaFile {
}

pub fn write_host_config(host: ColmenaHosts) -> String {
    let start_string = format!("
    {} = {{
    deployment = {{
    targetHost = '{}';
    ", host.name, host.ip);
    let middle_string = if host.remotebuilder {
        format!("
        targetHost = null
        allowLocalDeployment = true;
        buildOnTarget = true;
        ") 
    } else {
        format!("
        targetHost = taruser;
        buildOnTarget = false;
        keys = commonSSHKeys;
        ")
    }
    let end_string = format!("
        imports = [
        {}
        ];", for i in host.imports { 
        format!("{}\n", i)
    });
    let final_string = format!("
    {}
    {}
    }};
    {}
    }};
           ", 
           start_string,
           middle_string,
           end_string,
     );
    final_string
}
