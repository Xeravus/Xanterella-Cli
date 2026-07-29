use crate::engine::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_core_new() {
        let lexer = Lexer::new("", String::from("path.nix"));
        let empty_events: Vec<ParseEvent> = vec![];

        assert!(lexer.event.is_empty());
        assert!(!lexer.path.is_empty());

        assert_eq!(lexer.path, String::from("path.nix"));
        assert_eq!(lexer.event, empty_events);
    }
}
