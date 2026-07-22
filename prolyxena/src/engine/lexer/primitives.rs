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
    fn parse_expression(&mut self) -> Result<NixValue, String>;
    fn parse_operator(&mut self) -> Option<Operator>;
}

impl<'a> ParsePrimitves for Lexer<'a> {
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
            Some(&'(') => {
                self.chars.next();
                self.event.push(ParseEvent::StartGroup);
                let parsed_expr = self.parse_expression()?;
                let expr = NixValue::Group(Box::new(parsed_expr));
                self.skip_whitespace();
                if let Some(&')') = self.chars.peek() {
                    self.chars.next();
                    self.event.push(ParseEvent::EndGroup);
                    Ok(expr)
                } else {
                    Err(format!("Syntax-Fehler: Erwartetes ')' nach dem Ausdruck\n Datei: {}", &self.path))
                }
            },
            Some(&'$') => {
                self.chars.next();
                if let Some(&'{') = self.chars.peek() {
                    self.chars.next();
                    self.event.push(ParseEvent::StartAntiquotation);
                    let parsed_expr = self.parse_expression()?;
                    let expr = NixValue::Antiquotation(Box::new(parsed_expr));
                    self.skip_whitespace();
                    if let Some(&'}') = self.chars.peek() {
                        self.chars.next();
                        self.event.push(ParseEvent::EndAntiquotation);
                        Ok(expr)
                    } else {
                        Err(format!("Syntax-Fehler: Erwartetes '}}' nach der Antiquotation\n Datei: {}", &self.path))
                    }
                } else {
                    Err(format!("Syntax-Fehler: Erwartet '{{' nach '$' für eine Antiquotation\n Datei: {}", &self.path))
                }
            },
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_alphanumeric() || *c == '_' => self.parse_identifier(),
            None => Err(format!("Syntax-Fehler: Unerwaretes Ende der Datei \n Datei: {}", &self.path)),
            Some(c) => Err(format!("Syntax-Fehler: Unerwartetes Zeichen '{}' \n Datei: {}", c, &self.path)),
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
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Kommazahl: '{}'\nDatei: {}", value_str, &self.path)),
            }
        } else {
            match value_str.parse::<u64>() {
                Ok(int_val) =>  {
                    self.event.push(ParseEvent::EndNumber);
                    Ok(NixValue::Int(int_val))
                },
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Ganzzahl '{}'\nDatei: {}", value_str, &self.path)),
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
            return Err(format!("Syntax-Fehler: Unerwartet leerer Identifier \n Datei: {}", &self.path));
        }

        match word.as_str() {
            "let" => {
                self.parse_let_in()
            },
            "with" => {
                self.parse_with()
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

    fn parse_expression(&mut self) -> Result<NixValue, String> {
        self.event.push(ParseEvent::StartExpression);
        let mut left = self.parse_single_value()?;
        while let Some(op) = self.parse_operator() {
            let right = self.parse_single_value()?;
            left = NixValue::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            }
        }
        self.event.push(ParseEvent::EndExpression);
        Ok(left)
    }

    fn parse_operator(&mut self) -> Option<Operator> {
        self.skip_whitespace();
        let mut scout = &mut self.chars.clone();
        let first = &scout.next()?;
        match first {
            '+' => {
                if let Some(&'+') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.event.push(ParseEvent::StartOperator);
                    self.event.push(ParseEvent::EndOperator);
                    Some(Operator::Concat)
                } else {
                    self.chars.next();
                    self.event.push(ParseEvent::StartOperator);
                    self.event.push(ParseEvent::EndOperator);
                    Some(Operator::Add)
                }
            }
            '-' => {
                self.chars.next();
                self.event.push(ParseEvent::StartOperator);
                self.event.push(ParseEvent::EndOperator);
                Some(Operator::Sub)
            }
            '=' => {
                if let Some(&'=') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.event.push(ParseEvent::StartOperator);
                    self.event.push(ParseEvent::EndOperator);
                    Some(Operator::Equal)
                } else {
                    None
                }
            }
            '/' => {
                if let Some(&'/') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.event.push(ParseEvent::StartOperator);
                    self.event.push(ParseEvent::EndOperator);
                    Some(Operator::Merge)
                } else {
                    self.event.push(ParseEvent::StartOperator);
                    self.event.push(ParseEvent::EndOperator);
                    Some(Operator::Divide)
                }
            }
            _ => None
        }
    }
}

#[cfg(test)]
#[path = "primitives_test.rs"]
mod tests;
