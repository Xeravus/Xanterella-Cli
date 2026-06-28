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
    fn show_parse_timeline(&self);
}

impl<'a> ParseCore for Parser<'a> {
    fn skip_whitespace(&mut self) {
        self.event.push(ParseEvent::StartWhitespace);
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
        self.event.push(ParseEvent::EndWhitespace);
    }

    fn parse_value(&mut self) -> Result<NixValue, String> {
        self.event.push(ParseEvent::StartValue);
        let mut expr = self.parse_single_value()?;
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
        self.event.push(ParseEvent::EndValue);
        Ok(expr)
    }
    fn show_parse_timeline(&self) {
        for i in &self.event {
            match i {
                // Start
                ParseEvent::StartAttrSet => println!("Event: Starte Attribut Set"),
                ParseEvent::StartList => println!("Event: Starte Liste"),
                ParseEvent::StartLetIn => println!("Event: Starte Let-In"),
                ParseEvent::StartLambda => println!("Event: Starte Lambda"),
                ParseEvent::StartWith => println!("Event: Starte With"),
                ParseEvent::StartString => println!("Event: Starte String"),
                ParseEvent::StartPath => println!("Event: Starte Path"),
                ParseEvent::StartNumber => println!("Event: Starte Number"),
                ParseEvent::StartIdentifier => println!("Event: Starte Identifier"),
                ParseEvent::StartWhitespace => println!("Event: Starte Whitespace"),
                ParseEvent::StartValue => println!("Event: Starte Value"),
                // End 
                ParseEvent::EndAttrSet => println!("Event: Ende Attribut Set"),
                ParseEvent::EndList => println!("Event: End Liste"),
                ParseEvent::EndLetIn => println!("Event: End Let-In"),
                ParseEvent::EndLambda => println!("Event: End Lambda"),
                ParseEvent::EndWith => println!("Event: End With"),
                ParseEvent::EndString => println!("Event: End String"),
                ParseEvent::EndPath => println!("Event: End Path"),
                ParseEvent::EndNumber => println!("Event: End Number"),
                ParseEvent::EndIdentifier => println!("Event: End Identifier"),
                ParseEvent::EndWhitespace => println!("Event: End Whitespace"),
                ParseEvent::EndValue => println!("Event: End Whitespace"),
            }
            sleep(Duration::from_millis(10));
        }
    }
}
