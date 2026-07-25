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
    fn parse_indented_string(&mut self) -> Result<NixValue, String>;
    fn parse_application(&mut self) -> Result<NixValue, String>;
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
            Some(&'\'') => {
                self.chars.next();
                if let Some(&'\'') = self.chars.peek() {
                    self.chars.next();
                    self.log_event(ParseEvent::StartIndentedString);
                    self.parse_indented_string()
                } else {
                    Err(format!("Syntax-Fehler: Erwartet ''' um einen Indented String zu starten \nDatei: {} \nErwartet: Indented String", &self.path))
                }
            },
            Some(&'(') => {
                self.chars.next();
                self.log_event(ParseEvent::StartGroup);
                let parsed_expr = self.parse_expression()?;
                let expr = NixValue::Group(Box::new(parsed_expr));
                self.skip_whitespace();
                if let Some(&')') = self.chars.peek() {
                    self.chars.next();
                    self.log_event(ParseEvent::EndGroup);
                    Ok(expr)
                } else {
                    Err(format!("Syntax-Fehler: Erwartet ')' nach der Gruppe: '{:#?}' \nDatei: {} \nErwartet: Group", expr, &self.path))
                }
            },
            Some(&'$') => {
                self.chars.next();
                if let Some(&'{') = self.chars.peek() {
                    self.chars.next();
                    self.log_event(ParseEvent::StartAntiquotation);
                    let parsed_expr = self.parse_expression()?;
                    let expr = NixValue::Antiquotation(Box::new(parsed_expr));
                    self.skip_whitespace();
                    if let Some(&'}') = self.chars.peek() {
                        self.chars.next();
                        self.log_event(ParseEvent::EndAntiquotation);
                        Ok(expr)
                    } else {
                        Err(format!("Syntax-Fehler: Erwartet '}}' nach der Antiquotation \nDatei: {} \nErwartet: Antiquotation", &self.path))
                    }
                } else {
                    Err(format!("Syntax-Fehler: Erwartet '{{' nach '$' für eine Antiquotation \nDatei: {} \nErwartet: Antiquotation", &self.path))
                }
            },
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_alphanumeric() || *c == '_' => self.parse_identifier(),
            None => Err(format!("Syntax-Fehler: Unerwaretes Ende der Datei \nDatei: {} \nErwartet: Unknown", &self.path)),
            Some(c) => Err(format!("Syntax-Fehler: Unerwartetes Zeichen '{}' \nDatei: {} \nErwartet: Unknown", c, &self.path)),
        }
    }

    fn parse_path(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartPath);
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() || c == ';' {
                break;
            }
            string.push(c);
            self.chars.next();
        }
        self.log_event(ParseEvent::EndPath);
        Ok(NixValue::Path(string))
    }


    fn parse_string(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartString);
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
        self.log_event(ParseEvent::EndString);
        Ok(NixValue::Str(value))
    }

    fn parse_number(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartNumber);
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
                    self.log_event(ParseEvent::EndNumber);
                    Ok(NixValue::Float(float_val))
                    },
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Kommazahl: '{}' \nDatei: {} \nErwartet: Number(f64)", value_str, &self.path)),
            }
        } else {
            match value_str.parse::<u64>() {
                Ok(int_val) =>  {
                    self.log_event(ParseEvent::EndNumber);
                    Ok(NixValue::Int(int_val))
                },
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Ganzzahl '{}' \nDatei: {} \nErwartet: Number(u64)", value_str, &self.path)),
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartIdentifier);
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
            return Err(format!("Syntax-Fehler: Unerwartet leerer Identifier \nDatei: {} \nErwartet: Identifier", &self.path));
        }

        match word.as_str() {
            "let" => {
                self.log_event(ParseEvent::EndIdentifier);
                self.parse_let_in()
            },
            "with" => {
                self.log_event(ParseEvent::EndIdentifier);
                self.parse_with()
            },
            "true" => {
                self.log_event(ParseEvent::EndIdentifier);
                Ok(NixValue::Bool(true))
            }
            "false" => {
                self.log_event(ParseEvent::EndIdentifier);
                Ok(NixValue::Bool(false))
            }
            _ => {
                self.log_event(ParseEvent::EndIdentifier);
                Ok(NixValue::Identifier(word))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartExpression);
        let mut left = self.parse_application()?;
        while let Some(op) = self.parse_operator() {
            let right = self.parse_application()?;
            left = NixValue::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            }
        }
        self.log_event(ParseEvent::EndExpression);
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
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::Concat)
                } else {
                    self.chars.next();
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::Add)
                }
            }
            '-' => {
                self.chars.next();
                self.log_event(ParseEvent::StartOperator);
                self.log_event(ParseEvent::EndOperator);
                Some(Operator::Sub)
            }
            '=' => {
                if let Some(&'=') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::Equal)
                } else {
                    None
                }
            }
            '/' => {
                if let Some(&'/') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::Merge)
                } else {
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::Divide)
                }
            }
            _ => None
        }
    }

    fn parse_indented_string(&mut self) -> Result<NixValue, String> {
        let mut output = vec![];
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if let Some(&'$') = self.chars.peek() {
                let scout = &mut self.chars.clone();
                scout.next();
                if let Some(&'{') = scout.peek() {
                    if !&string.is_empty() {
                        output.push(StringFragment::Text(string.clone()));
                        string.clear();
                    }
                    let parsed_expr = self.parse_single_value()?;
                    let expr = StringFragment::Antiquotation(Box::new(parsed_expr));
                    output.push(expr)
                } else {
                    string.push(c);
                    self.chars.next();
                }
            } 
            if let Some(&'\'') = self.chars.peek() {
                string.push(c);
                self.chars.next();
                if let Some(&'\'') = self.chars.peek() {
                    string.push(c);
                    self.chars.next();
                    if let Some(&'\'') = self.chars.peek() {
                        string.push(c);
                        self.chars.next();
                    } else if let Some(&'$') = self.chars.peek() {
                        string.push(c);
                        self.chars.next();
                    } else {
                        string.pop();
                        string.pop();
                        if !&string.is_empty() {
                            output.push(StringFragment::Text(string.clone()));
                            string.clear();
                        }
                        break;
                    }
                } else {
                    string.push(c);
                    self.chars.next();
                }
            }

            if let Some(&s) = self.chars.peek() {
                string.push(s);
                self.chars.next();
            }
        }
        if !&string.is_empty() {
            output.push(StringFragment::Text(string.clone()));
        }
        if let Some(&';') = self.chars.peek() {
        } else {
            return Err(format!("Syntax-Fehler: Erwartet ';' nach dem Indented String \nDatei: {} \nErwartet: Indented String", &self.path));
        }
        self.log_event(ParseEvent::EndIndentedString);
        Ok(NixValue::IndStr(output))
    }
    fn parse_application(&mut self) -> Result<NixValue, String> {
        let mut expr = self.parse_single_value()?;
        loop {
            self.skip_whitespace();
            match self.chars.peek() {
                None => break,
                //Klammern
                Some(&';') => break,
                Some(&'}') => break,
                Some(&']') => break,
                Some(&')') => break,
                Some(&'=') => break,
                Some(&',') => break,
                //Operatoren
                Some(&'+') => break,
                Some(&'-') => break,
                Some(&'/') => break,
                Some(&'*') => break,
                Some(&'<') => break,
                Some(&'>') => break,
                Some(&'$') => break,
                _ => {
                    let arg = self.parse_single_value()?;
                    expr = NixValue::Apply(Box::new(expr), Box::new(arg));
                }
            }
        }
        Ok(expr)
    }
}

#[cfg(test)]
#[path = "primitives_test.rs"]
mod tests;
