use crate::formater::core::*;

#[cfg(test)]
mod tests {
    use crate::engine::lexer::core::ParseCore;

    use super::*;
    #[test]
    fn test_engine_formater_core_attr_set() {
        let content = "
        { 
        bb = true;
        ab = true;
        };
        ";

        let mut lexer = Lexer::new(content, String::from("path.nix"));
        let result1 = lexer.parse_value();
        let mut result2 = result1.clone();

        assert_eq!(result1, Ok(NixValue::AttrSet(IndexMap::from([
                        (String::from("bb"), NixValue::Bool(true)), 
                        (String::from("ab"), NixValue::Bool(true))
        ]))));

        result2.as_mut().unwrap().sort_ast();

        assert_eq!(result2, Ok(NixValue::AttrSet(IndexMap::from([
                        (String::from("ab"), NixValue::Bool(true)), (String::from("bb"), NixValue::Bool(true))
        ]))));
    }
}
