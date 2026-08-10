use crate::engine::core::{NixValue, StringFragment};

pub trait Sort {
    fn sort_ast(&mut self);
}

impl Sort for NixValue {
    fn sort_ast(&mut self) {
        match self {
            NixValue::AttrSet(map) => {
                map.sort_keys();
                for (_, value) in map.iter_mut() {
                    value.sort_ast();
                }
            }
            NixValue::Lambda(vec, _, body) => {
                vec.sort();
                body.sort_ast();
            }
            NixValue::LetIn(map, body) => {
                map.sort_keys();
                for (_, value) in map.iter_mut() {
                    value.sort_ast();
                }
                body.sort_ast();
            }
            NixValue::With(box1, box2) => {
                box1.sort_ast();
                box2.sort_ast();
            }
            NixValue::Apply(box1, box2) => {
                box1.sort_ast();
                box2.sort_ast();
            }
            NixValue::List(vec) => {
                // vec.sort();
                for value in vec.iter_mut() {
                    value.sort_ast();
                }
            }
            NixValue::IndStr(fragments) => {
                for fragment in fragments.iter_mut() {
                    if let StringFragment::Antiquotation(ast_box) = fragment {
                        ast_box.sort_ast();
                    }
                }
            }
            NixValue::Group(value) => {
                value.sort_ast();
            }
            NixValue::Antiquotation(value) => {
                value.sort_ast();
            }
            NixValue::BinaryOp { left, operator: _, right } => {
                left.sort_ast();
                right.sort_ast();
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::*;
    use crate::engine::core::*;
    use crate::engine::lexer::core::ParseCore;
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

        assert_eq!(
            result1,
            Ok(NixValue::AttrSet(IndexMap::from([
                (String::from("bb"), NixValue::Bool(true)),
                (String::from("ab"), NixValue::Bool(true))
            ])))
        );

        result2.as_mut().unwrap().sort_ast();

        assert_eq!(
            result2,
            Ok(NixValue::AttrSet(IndexMap::from([
                (String::from("ab"), NixValue::Bool(true)),
                (String::from("bb"), NixValue::Bool(true))
            ])))
        );
    }
}
