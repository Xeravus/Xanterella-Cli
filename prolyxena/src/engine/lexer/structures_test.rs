use crate::engine::lexer::structures::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_lexer_structures_parse_attr_set() {
        let content1 = "name = name;}";
        let content2 = "name = }";
        let content3 = "{";

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

        assert!(!matches!(result1, Ok(NixValue::List(_)))); 
    }
}
