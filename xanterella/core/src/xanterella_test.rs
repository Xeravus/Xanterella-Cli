use crate::xanterella::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_core_xanterella_set_core() {
        let data = Xanterella::new();

        assert!(matches!(
            data,
            Xanterella {
                path: _,
                home: _,
                fast: _,
                debug: _,
                automate: _,
                sender: _,
            }
        ));
        assert!(data.path.is_empty());
        assert!(data.home.is_empty());
        assert!(data.sender.is_none());

        assert_eq!(data.fast, false);
        assert_eq!(data.debug, false);
        assert_eq!(data.automate, false);
    }

    #[test]
    fn test_core_xanterella_set_path() {
        let mut data = Xanterella::new();

        let _ = data.set_path("test.path");

        assert!(!data.path.is_empty());
        assert_eq!(data.path, String::from("test.path"));
    }

    #[test]
    fn test_core_xanterella_set_home() {
        let mut data = Xanterella::new();

        data.set_home("test.path");

        assert!(!data.home.is_empty());
        assert_eq!(data.home, String::from("test.path"));
    }

    #[test]
    fn test_core_xanterella_set_fast() {
        let mut data = Xanterella::new();

        data.set_fast(true);

        assert_eq!(data.fast, true);
        assert_ne!(data.fast, false);
    }

    #[test]
    fn test_core_xanterella_set_debug() {
        let mut data = Xanterella::new();

        data.set_debug(true);

        assert_eq!(data.debug, true);
        assert_ne!(data.debug, false);
    }

    #[test]
    fn test_core_xanterella_set_automate() {
        let mut data = Xanterella::new();

        data.set_automate(true);

        assert_eq!(data.automate, true);
        assert_ne!(data.automate, false);
    }
}
