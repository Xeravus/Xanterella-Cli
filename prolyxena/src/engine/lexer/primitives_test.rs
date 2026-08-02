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
        let content1 = "{}";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_attr_set() {
        let content1 = "{}";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_list() {
        let content1 = "[test test test]";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_string() {
        let content1 = "\"test\"";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
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
    fn test_engine_lexer_primitives_parse_single_value_group() {
        let content1 = "(test)";
        let content2 = "(test";

        let mut data1 = Lexer::new(content1, String::from("path"));
        let mut data2 = Lexer::new(content2, String::from("path"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::Group(_))));
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_antiquotation() {
        let content1 = "${test}";
        let content2 = "${test";
        let content3 = "$test}";
        let content4 = "$test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();
        let result3 = data3.parse_single_value();
        let result4 = data4.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());

        assert!(matches!(result1, Ok(NixValue::Antiquotation(_))));
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_digit() {
        let content1 = "1";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
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
        let content = "test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_path();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Path(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
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
        let content1 = "1";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_number();
        let result2 = data2.parse_number();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::Int(_))));

        assert!(!matches!(result1, Ok(NixValue::Float(_))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_number_float() {
        let content1 = "1.0";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_number();
        let result2 = data2.parse_number();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::Float(_))));

        assert!(!matches!(result1, Ok(NixValue::Int(_))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
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
        let content1 = "with test; test";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::With(..))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_let_it() {
        let content1 = "let in test";
        let content2 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::LetIn(..))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_bool() {
        let content1 = "true";
        let content2 = "false";
        let content3 = ",";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();
        let result3 = data3.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err());

        assert!(matches!(result1, Ok(NixValue::Bool(_))));
        assert!(matches!(result2, Ok(NixValue::Bool(_))));

        assert!(matches!(result1, Ok(NixValue::Bool(true))));
        assert!(matches!(result2, Ok(NixValue::Bool(false))));

        assert!(!matches!(result1, Ok(NixValue::Bool(false))));
        assert!(!matches!(result2, Ok(NixValue::Bool(true))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
        assert!(!matches!(result2, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression() {
        let content1 = "test";
        let content2 = "test ++ test";
        let content3 = "test ++ ";
        let content4 = ";test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));

        let result1 = data1.parse_expression();
        let result2 = data2.parse_expression();
        let result3 = data3.parse_expression();
        let result4 = data4.parse_expression();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err());
        assert!(result4.is_err());

        assert!(matches!(result1, Ok(NixValue::Identifier(_))));
        assert!(matches!(result2, Ok(NixValue::BinaryOp { left: _, operator: Operator::Concat, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator() {
        let content1 = "++";
        let content2 = "+";
        let content3 = "=";
        let content4 = "r";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));

        let result1 = data1.parse_operator();
        let result2 = data2.parse_operator();
        let result3 = data3.parse_operator();
        let result4 = data4.parse_operator();

        assert!(result1.is_some());
        assert!(result2.is_some());
        assert!(result3.is_none());
        assert!(result4.is_none());

        assert_eq!(result1, Some(Operator::Concat));
        assert_eq!(result2, Some(Operator::Add));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_indented_string() {
        let content1 = "test'';";
        let content2 = "${test}'';";
        let content3 = "test${test}test'';";
        let content4 = "test$(test)test'';";
        let content5 = "test''$(test)test'';";
        let content6 = "test''";
        let content7 = "test';";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));
        let mut data5 = Lexer::new(content5, String::from("path.nix"));
        let mut data6 = Lexer::new(content6, String::from("path.nix"));
        let mut data7 = Lexer::new(content7, String::from("path.nix"));

        let result1 = data1.parse_indented_string();
        let result2 = data2.parse_indented_string();
        let result3 = data3.parse_indented_string();
        let result4 = data4.parse_indented_string();
        let result5 = data5.parse_indented_string();
        let result6 = data6.parse_indented_string();
        let result7 = data7.parse_indented_string();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        assert!(result4.is_ok());
        assert!(result5.is_ok());
        assert!(result6.is_err());
        assert!(result7.is_err());

        assert!(matches!(result1, Ok(NixValue::IndStr(_))));
        assert!(matches!(result2, Ok(NixValue::IndStr(_))));
        assert!(matches!(result3, Ok(NixValue::IndStr(_))));
        assert!(matches!(result4, Ok(NixValue::IndStr(_))));
        assert!(matches!(result5, Ok(NixValue::IndStr(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_application() {
        let content1 = "test test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));

        let result1 = data1.parse_application();

        assert!(result1.is_ok());

        assert!(matches!(result1, Ok(NixValue::Apply(..))));
    }
}
