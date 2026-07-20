use crate::engine::lexer::primitives::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_empty() {
        let content = "";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_error() {
        let content = ";";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_lambda() {
        let content = "{}";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_attr_set() {
        let content = "{}";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_list() {
        let content = "[test test test]";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_string() {
        let content = "\"test\"";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_path() {
        let content1 = ".test";
        let content2 = "/test";
        let content3 = "~test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();
        let result3 = data3.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_digit() {
        let content = "1";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_identifier() {
        let content = "test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }

    #[test]
    fn test_engine_lexer_primitives_parse_path() {
        let content1 = ".test";
        let content2 = "/test";
        let content3 = "~test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));

        let result1 = data1.parse_path();
        let result2 = data2.parse_path();
        let result3 = data3.parse_path();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        assert!(matches!(result1, Ok(NixValue::Path(_))));
        assert!(matches!(result2, Ok(NixValue::Path(_))));
        assert!(matches!(result3, Ok(NixValue::Path(_))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
        assert!(!matches!(result2, Ok(NixValue::AttrSet(_))));
        assert!(!matches!(result3, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_string() {
        let content = "test\"";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_string();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Str(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_number_int() {
        let content = "1";

        let mut data = Lexer::new(content, String::from("path.nix"));

        let result = data.parse_number();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Int(_))));

        assert!(!matches!(result, Ok(NixValue::Float(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_number_float() {
        let content = "1.0";

        let mut data = Lexer::new(content, String::from("path.nix"));

        let result = data.parse_number();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Float(_))));

        assert!(!matches!(result, Ok(NixValue::Int(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_word() {
        let content = "test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_identifier();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Identifier(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_with() {
        let content = "with test; test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_identifier();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::With(..))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_let_it() {
        let content = "let in test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_identifier();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::LetIn(..))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_bool() {
        let content1 = "true";
        let content2 = "false";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        assert!(matches!(result1, Ok(NixValue::Bool(_))));
        assert!(matches!(result2, Ok(NixValue::Bool(_))));

        assert!(matches!(result1, Ok(NixValue::Bool(true))));
        assert!(matches!(result2, Ok(NixValue::Bool(false))));

        assert!(!matches!(result1, Ok(NixValue::Bool(false))));
        assert!(!matches!(result2, Ok(NixValue::Bool(true))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
        assert!(!matches!(result2, Ok(NixValue::AttrSet(_))));
    }
}
