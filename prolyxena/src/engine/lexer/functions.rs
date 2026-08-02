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
        let mut depth = 1;
        scout.next();

        // while let Some(c) = scout.next() {
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
                if c.is_whitespace() || c.is_alphanumeric() || c == '_' || c == '-' {
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
        self.chars.next();
        let mut vec: Vec<String> = vec![];
        let mut alias = None;
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
        self.skip_whitespace();
        if let Some(&'@') = self.chars.peek() {
            self.chars.next();
            self.skip_whitespace();
            match self.parse_identifier()? {
                NixValue::Identifier(name) => {
                    alias = Some(name);
                }
                _ => {
                    return Err(format!(
                        "Syntax-Fehler: Gültiger Variablename nach '@' erwartet \nDatei: {} \nErwartet: Lambda",
                        self.path
                    ));
                }
            }
        }
        self.skip_whitespace();

        if let Some(&':') = self.chars.peek() {
            self.chars.next();
        } else {
            return Err(format!(
                "Syntax-Fehler: Erwartetes ':' nach den Funktions-Argumenten \nDatei: {} \nErwartet: Lambda",
                self.path
            ));
        }
        self.skip_whitespace();
        let body = self.parse_value()?;
        self.log_event(ParseEvent::EndLambda);
        Ok(NixValue::Lambda(vec, alias, Box::new(body)))
    }
}

#[cfg(test)]
#[path = "functions_test.rs"]
mod tests;
