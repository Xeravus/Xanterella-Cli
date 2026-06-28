use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::engine::lexer::core::*;
use crate::engine::core::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::functions::*;

pub trait ParseStructures {
    fn parse_attr_set(&mut self) -> Result<NixValue, String>;
    fn parse_list(&mut self) -> Result<NixValue, String>;
}

impl<'a> ParseStructures for Parser<'a> {
    fn parse_attr_set(&mut self) -> Result<NixValue, String> {
        self.chars.next();
        let mut map = HashMap::new();
        loop {
            self.skip_whitespace();
            if let Some(&'}') = self.chars.peek() {
                self.chars.next();
                break;
            }

            let mut key = String::new();
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() || c == '=' {
                    break;
                }
                key.push(c);
                self.chars.next();
            }

            if key.is_empty() {
                return Err("Syntax-Fehler: Leerer Key im AttrSet".to_string());
            }

            if key == "inherit" {
                loop {
                    self.skip_whitespace();
                    if let Some(&';') = self.chars.peek() {
                        self.chars.next();
                        break;
                    }

                    let mut inherit = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c.is_whitespace() || c == ';' {
                            break;
                        }
                        inherit.push(c);
                        self.chars.next();
                    }
                    if inherit.is_empty() {
                        return Err("Syntax-Fehler: Unerwartetes Zeichen im 'inherit' Statment".to_string());
                    }
                    map.insert(
                        inherit.clone(),
                        NixValue::Identifier(inherit)
                    );
                }
                continue;
            }
            self.skip_whitespace();

            if let Some(&'=') = self.chars.peek() {
                self.chars.next();
            } else {
                return Err(format!("Syntax-Fehler: Erwartetes '=' nach Key '{}'", key));
            }

            let value = self.parse_value()?;
            self.skip_whitespace();

            if let Some(&';') = self.chars.peek() {
                self.chars.next();
            } else {
                return Err(format!("Syntax-Fehler: Erwartetes ';' nach dem Wert von '{}'", key));
            }
            map.insert(key, value);
        }
        Ok(NixValue::AttrSet(map))
    }

    fn parse_list(&mut self) -> Result<NixValue, String> {
        self.chars.next();
        let mut output_vec: Vec<NixValue> = vec![];
        loop {
            self.skip_whitespace();
            if let Some(&']') = self.chars.peek() {
                self.chars.next();
                break;
            }

            let value = self.parse_value()?;
            output_vec.push(value);
        }
        Ok(NixValue::List(output_vec))
    }
}
