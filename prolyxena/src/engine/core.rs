use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum NixValue {
    AttrSet(HashMap<String, NixValue>),
    List(Vec<NixValue>),
    Str(String),
    Int(u64),
    Float(f64),
    Bool(bool),
    Identifier(String),
    LetIn(HashMap<String, NixValue>, Box<NixValue>),
    With(Box<NixValue>, Box<NixValue>),
    Lambda(Vec<String>, Option<String>, Box<NixValue>),
    Apply(Box<NixValue>, Box<NixValue>),
    Path(String),
}

#[derive(Debug, Clone)]
pub enum ParseEvent {
    StartAttrSet,
    EndAttrSet,
    StartList,
    EndList,
    StartLetIn,
    EndLetIn,
    FoundLambda,
    ParsedString,
    ParsedNumber,
    ParsedPath,
    ParsedWith,

pub struct Parser<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Parser {
            chars: content.chars().peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_einfaches_attr_set() {
        let code = "{ boot = \"grub\"; enable = true; }";
        let mut parser = Parser::new(code);
        let ast = parser.parse_value().unwrap();
        match ast {
            NixValue::AttrSet(map) => {
                assert!(map.contains_key("boot"), "Der Key 'boot' fehlt!");
                assert!(map.contains_key("enable"), "Der Key 'enable' fehlt!");
            }
            _ => panic!("Der Parser hat kein AttrSet erkannt, sondern: {:#?}", ast),
        }
    }

    #[test]
    fn test_lambda_funktion() {
        let code = "{ pkgs, config }: { boot = \"grub\"; }";
        let mut parser = Parser::new(code);
        let ast = parser.parse_value().unwrap();
        match ast {
            NixValue::Lambda(args, body) => {
                assert_eq!(args.len(), 2, "Es sollten genau 2 Argumente sein");
                assert_eq!(args[0], "pkgs");
                assert_eq!(args[1], "config");
                assert!(matches!(*body, NixValue::AttrSet(_)), "Body der Funktion ist kein AttrSet!");
            }
            _ => panic!("Der Parser hat kein Lambda erkannt, sondern: {:#?}", ast),
        }
    }

    #[test]
    fn test_let_in_block() {
        let code = "let hostname = \"nixos\"; in { name = hostname; }";
        let mut parser = Parser::new(code);
        let ast = parser.parse_value().unwrap();

        match ast {
            NixValue::LetIn(map, body) => {
                assert!(map.contains_key("hostname"), "Die Zuweisung 'hostname' fehlt!");
                assert!(matches!(*body, NixValue::AttrSet(_)), "Body des let-in ist kein AttrSet!");
            }
            _ => panic!("Der Parser hat kein LetIn erkannt, sondern: {:#?}", ast),
        }
    }
}
