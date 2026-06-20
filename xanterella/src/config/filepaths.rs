use std::path::Path;

use crate::utils::get::*;

pub enum OutPath {
    Full,
    Shortend,
    Last
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let filepath1 = "/home/cato/xanterella/config/test.nix";
        let filepath2 = "/home/cato/xanterella/config/test/test.nix";
        assert_eq!(convert_filepath(&filepath1, OutPath::Full, false), "/home/cato/xanterella/config/test.nix");
        assert_eq!(convert_filepath(&filepath1, OutPath::Full, true), "/home/cato/xanterella/config/test");
        assert_eq!(convert_filepath(&filepath2, OutPath::Full, false), "/home/cato/xanterella/config/test/test.nix");
        assert_eq!(convert_filepath(&filepath2, OutPath::Full, true), "/home/cato/xanterella/config/test/test");

        assert_eq!(convert_filepath(&filepath1, OutPath::Shortend, false), "test.nix");
        assert_eq!(convert_filepath(&filepath1, OutPath::Shortend, true), "test");
        assert_eq!(convert_filepath(&filepath2, OutPath::Shortend, false), "test/test.nix");
        assert_eq!(convert_filepath(&filepath2, OutPath::Shortend, true), "test/test");

        assert_eq!(convert_filepath(&filepath1, OutPath::Last, false), "test.nix");
        assert_eq!(convert_filepath(&filepath1, OutPath::Last, true), "test");
        assert_eq!(convert_filepath(&filepath2, OutPath::Last, false), "test.nix");
        assert_eq!(convert_filepath(&filepath2, OutPath::Last, true), "test");
    }
}
