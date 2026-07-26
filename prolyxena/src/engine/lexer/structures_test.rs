use crate::engine::lexer::structures::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_lexer_structures_parse_attr_set() {
        let content1 = "name = name;}";
        let content2 = "name = \"test\";}";
        let content3 = "name =";
        let content4 = "name = }";
        let content5 = "{";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));
        let mut data5 = Lexer::new(content5, String::from("path.nix"));
        
        let result1 = data1.parse_attr_set();
        let result2 = data2.parse_attr_set();
        let result3 = data3.parse_attr_set();
        let result4 = data4.parse_attr_set();
        let result5 = data5.parse_attr_set();
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err());
        assert!(result3.is_err());
        assert!(result5.is_err());
                
        assert!(matches!(result1, Ok(NixValue::AttrSet(_)))); 
        assert!(matches!(result2, Ok(NixValue::AttrSet(_)))); 

        assert!(!matches!(result1, Ok(NixValue::List(_)))); 
        assert!(!matches!(result2, Ok(NixValue::List(_)))); 
    }

    #[test]
    fn test_engine_lexer_parse_attr_set_inherit() {
        let content1 = "{inherit test;}";
        let content2 = "{inherit ;";
        let content3 = "{inherit };";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));

        let result1 = data1.parse_attr_set();
        let result2 = data2.parse_attr_set();
        let result3 = data3.parse_attr_set();

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());

        assert!(matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_strucutres_parse_list() {
        let content1 = "[ test ]";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));

        let result1 = data1.parse_list();

        assert!(result1.is_ok());

        assert!(matches!(result1, Ok(NixValue::List(_))));
    }
}
