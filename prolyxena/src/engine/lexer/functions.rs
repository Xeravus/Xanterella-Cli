use indexmap::IndexMap;

use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::lexer::primitives::*;

pub trait ParseFunctions {
    fn parse_let_in(&mut self) -> Result<NixValue, String>;
    fn parse_with(&mut self) -> Result<NixValue, String>;
    fn is_lambda_ahead(&self) -> bool;
    fn parse_lambda(&mut self) -> Result<NixValue, String>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum LamType {
    None,
    Nofix,
    Suffix,
    Prefix,
    Single,
}

impl<'a> ParseFunctions for Lexer<'a> {
    fn parse_let_in(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartLetIn);
        let mut map = IndexMap::new();
        loop {
            self.skip_whitespace();
            let mut key = String::new();
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() || c == '=' {
                    break;
                }
                key.push(c);
                self.chars.next();
            }

            if key.is_empty() {
                return Err(format!(
                    "Syntax-Fehler: Leerer Key im Let-In Statment \nDatei: {} \nErwartet: Let-In Statment",
                    self.path
                ));
            }

            if key == "in" {
                break;
            }

            self.skip_whitespace();

            if let Some(&'=') = self.chars.peek() {
                self.chars.next();
            } else {
                return Err(format!(
                    "Syntax-Fehler: Erwartet '=' nach Key '{}' \nDatei: {} \nErwartet: Let-In Statment",
                    key, self.path
                ));
            }

            let value = self.parse_value()?;
            self.skip_whitespace();

            if let Some(&';') = self.chars.peek() {
                self.chars.next();
            } else {
                return Err(format!(
                    "Syntax-Fehler: Erwartet ';' nach dem Wert '{}' \nDatei: {} \nErwartet: Let-In Statment",
                    key, self.path
                ));
            }
            map.insert(key, value);
        }
        self.skip_whitespace();
        let body = self.parse_value()?;
        self.log_event(ParseEvent::EndLetIn);
        Ok(NixValue::LetIn(map, Box::new(body)))
    }

    fn parse_with(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartWith);
        self.skip_whitespace();
        let namespace = self.parse_value()?;
        self.skip_whitespace();

        if let Some(&';') = self.chars.peek() {
            self.chars.next();
        } else {
            return Err(format!(
                "Syntax-Fehler: Erwartet ';' im 'with' Statment \nDatei: {} \nErwartet: With Statment",
                self.path
            ));
        }

        self.skip_whitespace();
        let body = self.parse_value()?;
        self.log_event(ParseEvent::EndWith);
        Ok(NixValue::With(Box::new(namespace), Box::new(body)))
    }

    fn is_lambda_ahead(&self) -> bool {
        let mut scout = self.chars.clone();

        if let Some(&c) = scout.peek() && (c.is_alphanumeric() || c == '_') {
            while let Some(&ch) = scout.peek() {
                if ch.is_alphanumeric() || c == '-' || c == '_' {
                    scout.next();
                } else {
                    break;
                }
            }

            while let Some(&ch) = scout.peek() {
                if ch.is_whitespace() {
                    scout.next();
                } else {
                    break;
                }
            }

            if let Some(&'@') = scout.peek() {
                scout.next();
                while let Some(&ch) = scout.peek() {
                    if ch.is_whitespace() {
                        scout.next();
                    } else {
                        break;
                    }
                }
            } else if let Some(':') = scout.peek() {
                return true;
            } else {
                scout = self.chars.clone();
            }
        }

        if let Some(&'{') = scout.peek() {
            scout.next();
        } else {
            return false;
        }

        let mut depth = 1;
        for c in scout.by_ref() {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
        }

        if depth != 0 {
            return false;
        }

        while let Some(&c) = scout.peek() {
            if c.is_whitespace() {
                scout.next();
            } else { 
                break;
            }
        }

        if let Some(&'@') = scout.peek() {
            scout.next();
            while let Some(&c) = scout.peek() {
                if c.is_whitespace() || c.is_alphanumeric() || c == '-' || c =='_' {
                    scout.next();
                } else {
                    break;
                }
            }
        }

        while let Some(&c) = scout.peek() {
            if c.is_whitespace() {
                scout.next();
            } else {
                break;
            }
        }

        matches!(scout.peek(), Some(&':'))
    }

    fn parse_lambda(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartLambda);

        let mut vec: Vec<String> = vec![];
        let mut alias = String::new();
        let mut lamtype = LamType::None;

        self.skip_whitespace();

        if let Some(&c) = self.chars.peek() && (c.is_alphanumeric() || c == '_') {
            let mut temp_alias = String::new();
            while let Some(&ch) = self.chars.peek() {
                if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                    temp_alias.push(ch);
                    self.chars.next();
                } else {
                    break;
                }
            }

            self.skip_whitespace();

            if let Some(&'@') = self.chars.peek() {
                self.chars.next();
                alias = temp_alias;
                lamtype = LamType::Prefix;
                self.skip_whitespace();
            } else if let Some(&':') = self.chars.peek() {
                alias = temp_alias;
                lamtype = LamType::Single;
            }

        }

        match lamtype {
            LamType::None | LamType::Prefix => {
                if let Some(&'{') = self.chars.peek() {
                    self.chars.next();
                } else {
                    return Err(format!("Syntax-Fehler: Erwartet '{{' für ein Lambda \nDatei: {} \nErwartet: Lambda", self.path));
                }

                loop {
                    self.skip_whitespace();
                    if let Some(&'}') = self.chars.peek() {
                        self.chars.next();
                        break;
                    }
                    let mut key = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c.is_whitespace() || c == ',' || c == '}' {
                            break;
                        }
                        key.push(c);
                        self.chars.next();
                    }
                    if !key.is_empty() {
                        vec.push(key);
                    }

                    self.skip_whitespace();

                    if let Some(&',') = self.chars.peek() {
                        self.chars.next();
                    }
                }
            }
            _ => { },
        }

        self.skip_whitespace();

        if lamtype == LamType::None {
            if let Some(&'@') = self.chars.peek() {
                self.chars.next();
                self.skip_whitespace();
                lamtype = LamType::Suffix;
                match self.parse_identifier()? {
                    NixValue::Identifier(name) => {
                        alias = name;
                    }
                    _ => {
                        return Err(format!(
                            "Syntax-Fehler: Gültiger Variablename nach '@' erwartet \nDatei: {} \nErwartet: Lambda \nAttribute:    Attr: {:#?}",
                            self.path, vec
                        ));
                    }
                }
            } else {
                lamtype = LamType::Nofix;
            }
        }

        self.skip_whitespace();

        if let Some(&':') = self.chars.peek() {
            self.chars.next();
        } else {
            return Err(format!(
                "Syntax-Fehler: Erwartetes ':' nach den Funktions-Argumenten \nDatei: {} \nErwartet: Lambda \nAttribute: \n    Attr: {:#?} \n    Alias: {}",
                self.path, vec, alias
            ));
        }
        self.skip_whitespace();
        let body = self.parse_value()?;
        self.log_event(ParseEvent::EndLambda);
        match lamtype {
            LamType::Nofix => Ok(NixValue::Lambda(LambdaTypes::Nofix(vec, Box::new(body)))),
            LamType::Suffix => Ok(NixValue::Lambda(LambdaTypes::Suffix(vec, alias, Box::new(body)))),
            LamType::Prefix => Ok(NixValue::Lambda(LambdaTypes::Prefix(vec, alias, Box::new(body)))),
            LamType::Single => Ok(NixValue::Lambda(LambdaTypes::Single(alias, Box::new(body)))),
            _ => Err(format!("Syntax-Fehler: Konnte Lambda nicht kategorisieren \nDatei: {} \nErwartet: Lambda \nAttribute: \n    Attr: {:#?} \n    Aias: {}", self.path, vec, alias )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_lexer_functions_parse_let_in() {
        let content1 = "in test";
        let content2 = "n test";
        let content3 = " test";
        let content4 = "in test; test";
        let content5 = "test test";

        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let mut data2 = Lexer::new(content2, String::from("path.nix"));
        let mut data3 = Lexer::new(content3, String::from("path.nix"));
        let mut data4 = Lexer::new(content4, String::from("path.nix"));
        let mut data5 = Lexer::new(content5, String::from("path.nix"));

        let result1 = data1.parse_let_in();
        let result2 = data2.parse_let_in();
        let result3 = data3.parse_let_in();
        let result4 = data4.parse_let_in();
        let result5 = data5.parse_let_in();

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_ok());
        assert!(result5.is_err());

        assert!(matches!(result1, Ok(NixValue::LetIn(..))));
        assert!(matches!(result3, Err(_)));

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
    fn test_engine_lexer_functions_is_lambda_ahead_nofix() {
        let content1 = "{test, }:";
        let data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.is_lambda_ahead();
        assert_eq!(result1, true);
    }

    #[test]
    fn test_engine_lexer_functions_parse_lambda_nofix() {
        let content1 = "{test, }: test";
        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.parse_lambda();

        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::Lambda(LambdaTypes::Nofix(..)))));
    }

    #[test]
    fn test_engine_lexer_functions_is_lambda_ahead_suffix() {
        let content1 = "{test, } @ inputs:";
        let data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.is_lambda_ahead();
        assert_eq!(result1, true);
    }

    #[test]
    fn test_engine_lexer_functions_parse_lambda_suffix() {
        let content1 = "{test, } @ inputs: test";
        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.parse_lambda();

        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::Lambda(LambdaTypes::Suffix(..)))));
    }

    #[test]
    fn test_engine_lexer_functions_is_lambda_ahead_prefix() {
        let content1 = "inputs @ {test, }:";
        let data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.is_lambda_ahead();
        assert_eq!(result1, true);
    }

    #[test]
    fn test_engine_lexer_functions_parse_lambda_prefix() {
        let content1 = "inputs @ {test, }: test";
        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.parse_lambda();

        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::Lambda(LambdaTypes::Prefix(..)))));
    }

    #[test]
    fn test_engine_lexer_functions_is_lambda_ahead_single() {
        let content1 = "test:";
        let data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.is_lambda_ahead();
        assert_eq!(result1, true);
    }

    #[test]
    fn test_engine_lexer_functions_parse_lambda_single() {
        let content1 = "test: test";
        let mut data1 = Lexer::new(content1, String::from("path.nix"));
        let result1 = data1.parse_lambda();

        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::Lambda(LambdaTypes::Single(..)))));
    }
}
