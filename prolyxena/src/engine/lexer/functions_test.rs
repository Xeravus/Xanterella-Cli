use crate::engine::lexer::functions::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_lexer_functions_parse_let_in() {
        let content1 = "in test";
        let content2 = "n test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_let_in();
        let result2 = data2.parse_let_in();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::LetIn(..))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_functions_parse_with() {
        let content1 = "test; test";
        let content2 = "test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_with();
        let result2 = data2.parse_with();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::With(..))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }
    
    #[test]
    fn test_engine_lexer_functions_is_lambda_ahead() {
        let content1 = "test, }:";
        let content2 = "}";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.is_lambda_ahead();
        let result2 = data2.is_lambda_ahead();

        assert_eq!(result1, true);
        assert_ne!(result1, false);

        assert_eq!(result2, false);
        assert_ne!(result2, true);
    }

    #[test]
    fn test_engine_lexer_functions_parse_lambda() {
        let content = "test, }: test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_lambda();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Lambda(..))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(..))));
    }
}
