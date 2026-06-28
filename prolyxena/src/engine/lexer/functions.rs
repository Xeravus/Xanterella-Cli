use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::engine::lexer::core::*;
use crate::engine::core::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::structures::*;

pub trait ParseFunctions {
    fn parse_let_in(&mut self) ->  Result<NixValue, String>;
    fn parse_with(&mut self) -> Result<NixValue, String>;
    fn is_lambda_ahead(&self) -> bool;
    fn parse_lambda(&mut self) -> Result<NixValue, String>;
}

impl<'a> ParseFunctions for Parser<'a> {
    fn parse_let_in(&mut self) ->  Result<NixValue, String> {
        let mut map = HashMap::new();
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
                return Err("Syntax-Fehler: Leerer Key im Let-In Statment".to_string());
            }

            if key == "in" {
                break;
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
                return Err(format!("Syntax-Fehler: Erwartetes ';' nach dem Wert von'{}'", key));
            }
            map.insert(key, value);
        }
        self.skip_whitespace();
        let body = self.parse_value()?;
        Ok(NixValue::LetIn(map, Box::new(body)))
    }

    fn parse_with(&mut self) -> Result<NixValue, String> {
        self.skip_whitespace();
        let namespace = self.parse_value()?;
        self.skip_whitespace();
        
        if let Some(&';') = self.chars.peek() {
            self.chars.next();
        } else {
            return Err("Syntax-Fehler: Erwartetes ';' im 'with' Statment".to_string());
        }

        self.skip_whitespace();
        let body = self.parse_value()?;
        Ok(NixValue::With(Box::new(namespace), Box::new(body)))
    }

    fn is_lambda_ahead(&self) -> bool {
        let mut scout = self.chars.clone();
        let mut depth = 1;
        scout.next();

        while let Some(c) = scout.next() {
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
        
        match scout.peek() {
            Some(&':') => true,
            _ => false,
        }
    }

    fn parse_lambda(&mut self) -> Result<NixValue, String> {
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
                },
                _ => return Err("Syntax-Fehler: Gültiger Variablename nach '@' erwartet".to_string()),
            }
        }
        self.skip_whitespace();

        if let Some(&':') = self.chars.peek() {
            self.chars.next();
        } else {
            return Err("Syntax-Erro: Erwartetes ':' nach den Funktions-Argumenten".to_string());
        }
        self.skip_whitespace();
        let body = self.parse_value()?;
        Ok(NixValue::Lambda(vec, alias, Box::new(body)))
    }
}
