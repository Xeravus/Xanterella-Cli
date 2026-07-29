use crate::utils::git::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_utils_git_git_commit() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        xanterella1.debug = true;
        xanterella2.path = "/test".to_string();

        let result1 = xanterella1.git_commit("Test");
        let result2 = xanterella2.git_commit("Test");

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    #[test]
    fn test_utils_git_git_checkout() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        xanterella1.debug = true;
        xanterella2.path = "/test".to_string();

        let result1 = xanterella1.git_checkout(Branches::Main);
        let result2 = xanterella2.git_checkout(Branches::Main);

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    #[test]
    fn test_utils_git_git_merge() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        xanterella1.debug = true;
        xanterella2.path = "/test".to_string();

        let result1 = xanterella1.git_merge();
        let result2 = xanterella2.git_merge();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }

    #[test]
    fn test_utils_git_git_pr() {
        let mut xanterella1 = Xanterella::new();

        xanterella1.debug = true;

        let result1 = xanterella1.git_merge();

        assert!(result1.is_ok());
    }
}
