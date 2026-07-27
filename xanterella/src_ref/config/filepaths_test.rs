use std::path::PathBuf;

use crate::config::filepaths::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let filepath1 = PathBuf::from(get_path(Paths::Nixconf)).join("test.nix").display().to_string();
        let filepath2 = PathBuf::from(get_path(Paths::Nixconf)).join("test").join("test.nix").display().to_string();
        assert!(convert_filepath(&filepath1, OutPath::Full, false).contains("/xanterella/config/test.nix"));
        assert!(convert_filepath(&filepath1, OutPath::Full, true).contains("/xanterella/config/test"));
        assert!(convert_filepath(&filepath2, OutPath::Full, false).contains("/xanterella/config/test/test.nix"));
        assert!(convert_filepath(&filepath2, OutPath::Full, true).contains("/xanterella/config/test/test"));

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
