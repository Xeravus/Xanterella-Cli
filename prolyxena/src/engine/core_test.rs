use std::sync::mpsc;

use crate::engine::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_core_new() {
        let lexer = Lexer::new("", String::from("path.nix"));

        assert!(!lexer.path.is_empty());

        assert_eq!(lexer.path, String::from("path.nix"));
    }

    #[test]
    fn test_engine_core_new_trans() {
        let (tx, rx) = mpsc::channel();
        let lexer = Lexer::new_trans("", String::from("path.nix"), tx);

        assert!(!lexer.path.is_empty());

        assert_eq!(lexer.path, String::from("path.nix"));
    }
}
