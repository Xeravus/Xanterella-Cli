use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::engine::core::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::structures::*;
use crate::engine::lexer::functions::*;

pub trait ParseCore {
    fn skip_whitespace(&mut self);
    fn parse_value(&mut self) -> Result<NixValue, String>;
}

impl<'a> ParseCore for Parser<'a> {
    fn skip_whitespace(&mut self) {
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
    }

    fn parse_value(&mut self) -> Result<NixValue, String> {
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
        Ok(expr)
    }
}
