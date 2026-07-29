
use crate::engine::core::*;
use crate::engine::lexer::primitives::*;

pub trait ParseCore {
    fn skip_whitespace(&mut self);
    fn parse_value(&mut self) -> Result<NixValue, String>;
}

impl<'a> ParseCore for Lexer<'a> {
    fn skip_whitespace(&mut self) {
        // self.log_event(ParseEvent::StartWhitespace);
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
        // self.log_event(ParseEvent::EndWhitespace);
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
}

#[cfg(test)]
#[path = "core_test.rs"]
mod tests;
