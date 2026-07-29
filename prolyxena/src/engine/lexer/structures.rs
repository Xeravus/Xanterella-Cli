use std::collections::HashMap;

use crate::engine::core::*;
use crate::engine::lexer::core::*;

pub trait ParseStructures {
    fn parse_attr_set(&mut self) -> Result<NixValue, String>;
    fn parse_list(&mut self) -> Result<NixValue, String>;
}

impl<'a> ParseStructures for Lexer<'a> {
    fn parse_attr_set(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartAttrSet);
        self.chars.next();
        let mut map = HashMap::new();
        loop {
            self.skip_whitespace();
            if let Some(&'}') = self.chars.peek() {
                self.chars.next();
                break;
            }

            let mut key = String::new();
            let mut quotes = false;
            while let Some(&c) = self.chars.peek() {
                if c == '"' {
                    quotes = !quotes;
                }
                if !quotes && (c.is_whitespace() || c == '=') {
                    break;
                }
                key.push(c);
                self.chars.next();
            }

            if key.is_empty() {
                return Err(format!(
                    "Syntax-Fehler: Leerer Key im AttrSet \nDatei: {} \nErwartet: Attribut Set",
                    self.path
                ));
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
                        return Err(format!(
                            "Syntax-Fehler: Unerwartetes Zeichen im 'inherit' Statment \nDatei: {} \nErwartet: Attribut Set(Inherit Statment)",
                            self.path
                        ));
                    }
                    map.insert(inherit.clone(), NixValue::Identifier(inherit));
                }
                continue;
            }
            self.skip_whitespace();

            if let Some(&'=') = self.chars.peek() {
                self.chars.next();
            } else {
                return Err(format!(
                    "Syntax-Fehler: Erwartet '=' nach Key '{}' \nDatei: {} \nErwartet: Attribut Set",
                    key, self.path
                ));
            }

            let value = self.parse_value()?;
            self.skip_whitespace();

            if let Some(&';') = self.chars.peek() {
                self.chars.next();
            } else {
                return Err(format!(
                    "Syntax-Fehler: Erwartet ';' nach dem Wert von '{}' \nDatei: {} \nErwartet: Attribut Set",
                    key, self.path
                ));
            }
            map.insert(key, value);
        }
        self.log_event(ParseEvent::EndAttrSet);
        Ok(NixValue::AttrSet(map))
    }

    fn parse_list(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartList);
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
        self.log_event(ParseEvent::EndList);
        Ok(NixValue::List(output_vec))
    }
}

#[cfg(test)]
#[path = "structures_test.rs"]
mod tests;
