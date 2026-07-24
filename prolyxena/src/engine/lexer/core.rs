use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;
use std::thread::sleep;
use std::time::Duration;

use crate::engine::core::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::structures::*;
use crate::engine::lexer::functions::*;

pub trait ParseCore {
    fn skip_whitespace(&mut self);
    fn parse_value(&mut self) -> Result<NixValue, String>;
    // fn show_parse_timeline(&self);
}

impl<'a> ParseCore for Lexer<'a> {
    fn skip_whitespace(&mut self) {
        self.log_event(ParseEvent::StartWhitespace);
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_whitespace() => {
                    self.chars.next();
                }
                Some(&'#') => {
                    self.chars.next();
                    while let Some(&comment) = self.chars.peek() {
                        if comment == '\n' {
                            self.chars.next();
                            break;
                        } else {
                            self.chars.next();
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        }
        self.log_event(ParseEvent::EndWhitespace);
    }

    fn parse_value(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartValue);
        let mut expr = self.parse_expression()?;
        loop {
            self.skip_whitespace();
            match self.chars.peek() {
                Some(&';') => {
                    break;
                }
                Some(&'}') => {
                    break;
                }
                Some(&']') => {
                    break;
                }
                Some(&')') => {
                    break;
                }
                Some(&'=') => {
                    break;
                }
                None => {
                    break;
                }
                _ => {
                    let arg = self.parse_single_value()?;
                    expr = NixValue::Apply(Box::new(expr), Box::new(arg));
                }
            }
        }
        self.log_event(ParseEvent::EndValue);
        Ok(expr)
    }

    /*
    fn show_parse_timeline(&self) {
        let mut indent = 0;
        for i in &self.event {
            let (is_start, name) = match i {
                ParseEvent::StartAttrSet => (true, "Attribut Set"),
                ParseEvent::EndAttrSet => (false, "Attribut Set"),

                ParseEvent::StartList => (true, "Liste"),
                ParseEvent::EndList => (false, "Liste"),

                ParseEvent::StartLetIn => (true, "LetIn"),
                ParseEvent::EndLetIn => (false, "LetIn"),

                ParseEvent::StartLambda => (true, "Lambda"),
                ParseEvent::EndLambda => (false, "Lambda"),

                ParseEvent::StartWith => (true, "With"),
                ParseEvent::EndWith => (false, "With"),

                ParseEvent::StartString => (true, "String"),
                ParseEvent::EndString => (false, "String"),

                ParseEvent::StartPath => (true, "Path"),
                ParseEvent::EndPath => (false, "Path"),

                ParseEvent::StartNumber => (true, "Number"),
                ParseEvent::EndNumber => (false, "Number"),

                ParseEvent::StartExpression => (true, "Expression"),
                ParseEvent::EndExpression => (false, "Expression"),

                ParseEvent::StartOperator => (true, "Operator"),
                ParseEvent::EndOperator => (false, "Operator"),

                ParseEvent::StartIdentifier => (true, "Identifier"),
                ParseEvent::EndIdentifier => (false, "Identifier"),

                ParseEvent::StartWhitespace => (true, "Whitespace"),
                ParseEvent::EndWhitespace => (false, "Whitespace"),

                ParseEvent::StartValue => (true, "Value"),
                ParseEvent::EndValue => (false, "Value"),

                ParseEvent::StartGroup => (true, "Group"),
                ParseEvent::EndGroup => (false, "Group"),

                ParseEvent::StartAntiquotation => (true, "Antiquotation"),
                ParseEvent::EndAntiquotation => (false, "Antiquotation"),

                ParseEvent::StartIndentedString => (true, "Intented String"),
                ParseEvent::EndIndentedString => (false, "Intented String"),
            };
            if !is_start && indent > 0 {
                indent -= 1;
            }

            let indent_string = "  ".repeat(indent);
            if is_start {
                println!("{}├── Starte {}", indent_string, name);
                indent += 1;
            } else {
                println!("{}└── Schließe {}", indent_string, name);
            }
            sleep(Duration::from_millis(15));
        }
    }
    */
}
