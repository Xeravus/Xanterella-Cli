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
}
