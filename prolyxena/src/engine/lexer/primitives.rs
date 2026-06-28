use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::engine::lexer::core::*;
use crate::engine::core::*;
use crate::engine::lexer::structures::*;
use crate::engine::lexer::functions::*;

pub trait ParsePrimitves {
    fn parse_single_value(&mut self) -> Result<NixValue, String>;
    fn parse_path(&mut self) -> Result<NixValue, String>;
    fn parse_string(&mut self) -> Result<NixValue, String>;
    fn parse_number(&mut self) -> Result<NixValue, String>;
    fn parse_identifier(&mut self) -> Result<NixValue, String>;
}

impl<'a> ParsePrimitves for Parser<'a> {
    fn parse_single_value(&mut self) -> Result<NixValue, String> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some(&'{') => {
                if self.is_lambda_ahead() {
                    self.parse_lambda()
                } else {
                    self.parse_attr_set()
                }
            },
            Some(&'[') => self.parse_list(),
            Some(&'"') => self.parse_string(),
            Some(&'.') => self.parse_path(),
            Some(&'/') => self.parse_path(),
            Some(&'~') => self.parse_path(),
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_alphanumeric() || *c == '_' => self.parse_identifier(),
            None => Err("Syntax-Fehler: Unerwaretes Ende der Datei".to_string()),
            Some(c) => Err(format!("Syntax-Fehler: Unerwartetes Zeichen '{}'", c)),
        }
    }

    fn parse_path(&mut self) -> Result<NixValue, String> {
        self.event.push(ParseEvent::StartPath);
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() || c == ';' {
                self.chars.next();
                break;
            }
            string.push(c);
            self.chars.next();
        }
        self.event.push(ParseEvent::EndPath);
        Ok(NixValue::Path(string))
    }


    fn parse_string(&mut self) -> Result<NixValue, String> {
        self.event.push(ParseEvent::StartString);
        self.chars.next();
        let mut value = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                self.chars.next();
                break;
            } else {
                value.push(c);
                self.chars.next();
            }
        }
        self.event.push(ParseEvent::EndString);
        Ok(NixValue::Str(value))
    }

    fn parse_number(&mut self) -> Result<NixValue, String> {
        self.event.push(ParseEvent::StartNumber);
        let mut value_str = String::new();
        let mut is_float = false;
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                value_str.push(c);
                self.chars.next();
            } else if c == '.' && !is_float {
                is_float = true;
                value_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        if is_float {
            match value_str.parse::<f64>() {
                Ok(float_val) => {
                    self.event.push(ParseEvent::EndNumber);
                    Ok(NixValue::Float(float_val))
                    },
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Kommazahl: '{}'", value_str)),
            }
        } else {
            match value_str.parse::<u64>() {
                Ok(int_val) =>  {
                    self.event.push(ParseEvent::EndNumber);
                    Ok(NixValue::Int(int_val))
                },
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Ganzzahl '{}'", value_str)),
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<NixValue, String> {
        self.event.push(ParseEvent::StartIdentifier);
        let mut word = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
                word.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        if word.is_empty() {
            return Err("Syntax-Fehler: Unerwartet leerer Identifier".to_string());
        }

        match word.as_str() {
            "let" => {
                self.parse_let_in()
                    .map_err(|_| "Syntax-Fehler: Unerwartetes Let-In Statment".to_string())
            },
            "with" => {
                self.parse_with()
                    .map_err(|_| "Syntax-Fehler: Unerwartetes 'With' Statment".to_string())
                
            },
            "true" => {
                self.event.push(ParseEvent::EndIdentifier);
                Ok(NixValue::Bool(true))
            }
            "false" => {
                self.event.push(ParseEvent::EndIdentifier);
                Ok(NixValue::Bool(false))
            }
            _ => {
                self.event.push(ParseEvent::EndIdentifier);
                Ok(NixValue::Identifier(word))
            }
        }
    }
}
