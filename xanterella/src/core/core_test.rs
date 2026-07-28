use crate::core::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_core_core_new() {
        let data = Xanterella::new();

        assert!(matches!(data, Xanterella { path: _, home: _, fast: _, debug: _, automate: _, }));
        assert!(matches!(&data.path, String));
        assert!(matches!(&data.home, String));
        assert!(matches!(data.fast, bool));
        assert!(matches!(data.debug, bool));
        assert!(matches!(data.automate, bool));

        assert!(data.path.is_empty());
        assert!(data.home.is_empty());

        assert_eq!(data.fast, false);
        assert_eq!(data.debug, false);
        assert_eq!(data.automate, false);
    }

    #[test]
    fn test_core_set_path() {
        let mut data = Xanterella::new();

        data.set_path("test.path");

        assert!(matches!(&data.path, String));
        assert!(!data.path.is_empty());
        assert_eq!(data.path, String::from("test.path"));
    }

    #[test]
    fn test_core_set_home() {
        let mut data = Xanterella::new();

        data.set_home("test.path");

        assert!(matches!(&data.home, String));
        assert!(!data.home.is_empty());
        assert_eq!(data.home, String::from("test.path"));
    }

    #[test]
    fn test_core_set_fast() {
        let mut data = Xanterella::new();

        data.set_fast(true);

        assert!(matches!(data.fast, bool));
        assert_eq!(data.fast, true);
        assert_ne!(data.fast, false);
    }

    #[test]
    fn test_core_set_debug() {
        let mut data = Xanterella::new();

        data.set_debug(true);

        assert!(matches!(data.debug, bool));
        assert_eq!(data.debug, true);
        assert_ne!(data.debug, false);
    }

    #[test]
    fn test_core_set_automate() {
        let mut data = Xanterella::new();

        data.set_automate(true);

        assert!(matches!(data.automate, bool));
        assert_eq!(data.automate, true);
        assert_ne!(data.automate, false);
    }
}
