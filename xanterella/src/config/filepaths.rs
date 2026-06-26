use std::path::{Path, PathBuf};

use crate::utils::get::*;

pub enum OutPath {
    Full,
    Shortend,
    Last
}

pub enum Target {
    Colmena,
    Hosts,
}

pub fn convert_filepath(input: &str, pathtype: OutPath, stem: bool) -> String {
    let path = Path::new(input);
    let result = match pathtype {
        OutPath::Full => {
            input.to_string().to_owned()
        }
        OutPath::Shortend => {
            match path.strip_prefix(get_path(Paths::Nixconf)) {
                Ok(stripped) => stripped.to_string_lossy().into_owned(),
                Err(_) => path.to_string_lossy().into_owned(),
            }
        }
        OutPath::Last => {
            path.file_name()
                .map(|i| i.to_string_lossy().into_owned())
                .unwrap_or_else(|| input.to_string())
        }
    };
    if stem {
        let result_path = Path::new(&result);
        return result_path.with_extension("").to_string_lossy().into_owned();
    }
    result
}

pub fn convert_path(input: &str, target: Target) -> String {
    let result = match target {
        Target::Colmena => PathBuf::from(input).join("colmena-hosts.nix"),
        Target::Hosts => PathBuf::from(input).join("hosts"),
    };
    result.display().to_string()
}

#[cfg(test)]
#[path = "filepaths_test.rs"]
mod tests; 
