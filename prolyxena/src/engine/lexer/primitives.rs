use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::lexer::functions::*;
use crate::engine::lexer::structures::*;

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
            }
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
                    Err(format!(
                        "Syntax-Fehler: Erwartet ''' um einen Indented String zu starten \nDatei: {} \nErwartet: Indented String",
                        self.path
                    ))
                }
            }
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
                    Err(format!(
                        "Syntax-Fehler: Erwartet ')' nach der Gruppe: '{:#?}' \nDatei: {} \nErwartet: Group",
                        expr, self.path
                    ))
                }
            }
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
                        Err(format!(
                            "Syntax-Fehler: Erwartet '}}' nach der Antiquotation \nDatei: {} \nErwartet: Antiquotation",
                            self.path
                        ))
                    }
                } else {
                    Err(format!(
                        "Syntax-Fehler: Erwartet '{{' nach '$' für eine Antiquotation \nDatei: {} \nErwartet: Antiquotation",
                        self.path
                    ))
                }
            }
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_alphanumeric() || *c == '_' || *c == '!' => {
                if self.is_lambda_ahead() {
                    self.parse_lambda()
                } else {
                    self.parse_identifier()
                }
            }
            None => {
                Err(format!("Syntax-Fehler: Unerwaretes Ende der Datei \nDatei: {} \nErwartet: Unknown", self.path))
            }
            Some(c) => {
                Err(format!("Syntax-Fehler: Unerwartetes Zeichen '{}' \nDatei: {} \nErwartet: Unknown", c, self.path))
            }
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
                }
                Err(_) => Err(format!(
                    "Syntax-Fehler: Ungültige Kommazahl: '{}' \nDatei: {} \nErwartet: Number(f64)",
                    value_str, self.path
                )),
            }
        } else {
            match value_str.parse::<u64>() {
                Ok(int_val) => {
                    self.log_event(ParseEvent::EndNumber);
                    Ok(NixValue::Int(int_val))
                }
                Err(_) => Err(format!(
                    "Syntax-Fehler: Ungültige Ganzzahl '{}' \nDatei: {} \nErwartet: Number(u64)",
                    value_str, self.path
                )),
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<NixValue, String> {
        self.log_event(ParseEvent::StartIdentifier);
        let mut word = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' || c == '!' {
                word.push(c);
                self.chars.next();
            } else if c == '$' {
                word.push(c);
                self.chars.next();
                if let Some(&'{') = self.chars.peek() {
                    word.push('{');
                    self.chars.next();
                    self.log_event(ParseEvent::StartAntiquotation);
                    while let Some(&d) = self.chars.peek() {
                        if d.is_alphanumeric() || d == '_' || d == '-' || d == '.' {
                            word.push(d);
                            self.chars.next();
                        } else if d == '}' {
                            word.push(d);
                            self.chars.next();
                            break;
                        } else {
                            return Err(format!(
                                "Syntax-Fehler: Unerwartetes Zeichen: '{}' \nDatei: {} \nErwartet: Antiquotation(Antiquotation inside an Identifier)",
                                c, self.path
                            ));
                        }
                    }
                } else {
                    return Err(format!(
                        "Syntax-Fehler: Erwartet '{{' nach einem '$' für eine Antiquotation \nDatei: {} \nErwartet: Antiquotation(Antiquotation inside an Indentifier)",
                        self.path
                    ));
                }
            } else {
                break;
            }
        }
        if word.is_empty() {
            return Err(format!(
                "Syntax-Fehler: Unerwartet leerer Identifier \nDatei: {} \nErwartet: Identifier",
                self.path
            ));
        }

        match word.as_str() {
            "let" => {
                self.log_event(ParseEvent::EndIdentifier);
                self.parse_let_in()
            }
            "with" => {
                self.log_event(ParseEvent::EndIdentifier);
                self.parse_with()
            }
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
            left = NixValue::BinaryOp { left: Box::new(left), operator: op, right: Box::new(right) }
        }
        self.log_event(ParseEvent::EndExpression);
        Ok(left)
    }

    fn parse_operator(&mut self) -> Option<Operator> {
        self.skip_whitespace();
        let scout = &mut self.chars.clone();
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
            '!' => {
                if let Some(&'=') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::Unequal)
                } else {
                    None
                }
            }
            '&' => {
                if let Some(&'&') = scout.peek() {
                    self.chars.next();
                    self.chars.next();
                    self.log_event(ParseEvent::StartOperator);
                    self.log_event(ParseEvent::EndOperator);
                    Some(Operator::And)
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
            _ => None,
        }
    }

    fn parse_indented_string(&mut self) -> Result<NixValue, String> {
        let mut output = vec![];
        let mut string = String::new();

        while let Some(&c) = self.chars.peek() {
            match c {
                '$' => {
                    self.chars.next();
                    if let Some(&'{') = self.chars.peek() {
                        self.chars.next();
                        if !string.is_empty() {
                            output.push(StringFragment::Text(string.clone()));
                            string.clear();
                        }

                        let parsed_expr = self.parse_expression()?;
                        let expr = StringFragment::Antiquotation(Box::new(parsed_expr));
                        output.push(expr);
                        self.skip_whitespace();
                        if let Some(&'}') = self.chars.peek() {
                            self.chars.next();
                        } else {
                            return Err(format!(
                                "Syntax-Fehler: Erwartet '}}' am Ende der Antiquotation im Indented String: '{:#?}' \nDatei: {} \nErwartet: Indented String(Antiquotation)",
                                output, self.path
                            ));
                        }
                    } else {
                        string.push('$');
                    }
                }
                '\'' => {
                    self.chars.next();
                    if let Some(&'\'') = self.chars.peek() {
                        self.chars.next();

                        if let Some(&'\'') = self.chars.peek() {
                            self.chars.next();
                            string.push('\'');
                            string.push('\'');
                            string.push('\'');
                        } else if let Some(&'$') = self.chars.peek() {
                            self.chars.next();
                            string.push('\'');
                            string.push('\'');
                            string.push('$');
                        } else {
                            if !string.is_empty() {
                                output.push(StringFragment::Text(string.clone()));
                                string.clear();
                            }
                            break;
                        }
                    } else {
                        string.push('\'');
                    }
                }
                _ => {
                    string.push(c);
                    self.chars.next();
                }
            }
        }

        if let Some(&';') = self.chars.peek() {
        } else {
            return Err(format!(
                "Syntax-Fehler: Erwartet ';' nach dem Indented String \nDatei: {} \nErwartet: Indented String",
                self.path
            ));
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
                Some(&'!') => break,
                Some(&'&') => break,
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
mod tests {
    use super::*;
    fn setup_lexer(content: &str) -> Lexer {
        Lexer::new(content, String::from("path.nix"))
    }

    #[test]
    fn test_engine_lexer_primitives_parse_single_value_empty() {
        let mut data = setup_lexer("");
        let result = data.parse_single_value();

        assert!(result.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_error() {
        let mut data = setup_lexer(";");
        let result = data.parse_single_value();

        assert!(result.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_lambda() {
        let mut data1 = setup_lexer("{}");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_attr_set() {
        let mut data1 = setup_lexer("{}");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_list() {
        let mut data1 = setup_lexer("[test test test]");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_string() {
        let mut data1 = setup_lexer("\"test\"");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_path() {
        let mut data1 = setup_lexer(".test");
        let mut data2 = setup_lexer("/test");
        let mut data3 = setup_lexer("~test");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();
        let result3 = data3.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_group() {
        let mut data1 = setup_lexer("(test)");
        let mut data2 = setup_lexer("(test");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::Group(_))));
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_antiquotation() {
        let mut data1 = setup_lexer("${test}");
        let mut data2 = setup_lexer("${test");
        let mut data3 = setup_lexer("$test}");
        let mut data4 = setup_lexer("$test");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();
        let result3 = data3.parse_single_value();
        let result4 = data4.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());

        assert!(matches!(result1, Ok(NixValue::Antiquotation(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_single_value_antiquotation_apply() {
        let mut data1 = setup_lexer("${test test}");
        let mut data2 = setup_lexer("${test test");
        let mut data3 = setup_lexer("$test test}");
        let mut data4 = setup_lexer("$test test");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();
        let result3 = data3.parse_single_value();
        let result4 = data4.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());

        assert!(matches!(result1, Ok(NixValue::Antiquotation(_))));
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_digit() {
        let mut data1 = setup_lexer("1");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_single_value();
        let result2 = data2.parse_single_value();

        assert!(result1.is_ok());
        assert!(result2.is_err());
    }
    #[test]
    fn test_engine_lexer_primitives_parse_single_value_identifier() {
        let content = "test";
        let mut data = Lexer::new(content, String::from("path.nix"));
        let result = data.parse_single_value();

        assert!(result.is_ok());
    }

    #[test]
    fn test_engine_lexer_primitives_parse_path() {
        let mut data = setup_lexer("test");
        let result = data.parse_path();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Path(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_string() {
        let mut data = setup_lexer("test\"");
        let result = data.parse_string();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Str(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_number_int() {
        let mut data1 = setup_lexer("1");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_number();
        let result2 = data2.parse_number();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::Int(_))));

        assert!(!matches!(result1, Ok(NixValue::Float(_))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_number_float() {
        let mut data1 = setup_lexer("1.0");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_number();
        let result2 = data2.parse_number();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::Float(_))));

        assert!(!matches!(result1, Ok(NixValue::Int(_))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_word() {
        let mut data = setup_lexer("test");
        let result = data.parse_identifier();

        assert!(result.is_ok());

        assert!(matches!(result, Ok(NixValue::Identifier(_))));

        assert!(!matches!(result, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_with() {
        let mut data1 = setup_lexer("with test; test");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::With(..))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_let_it() {
        let mut data1 = setup_lexer("let in test");
        let mut data2 = setup_lexer(",");

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result1, Ok(NixValue::LetIn(..))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_antiquotation() {
        let mut data1 = setup_lexer("le${pkgs}");
        let result1 = data1.parse_identifier();

        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::Identifier(..))));
        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_identifier_bool() {
        let mut data1 = setup_lexer("true");
        let mut data2 = setup_lexer("false");
        let mut data3 = setup_lexer(",");

        let result1 = data1.parse_identifier();
        let result2 = data2.parse_identifier();
        let result3 = data3.parse_identifier();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_err());

        assert!(matches!(result1, Ok(NixValue::Bool(_))));
        assert!(matches!(result2, Ok(NixValue::Bool(_))));

        assert!(matches!(result1, Ok(NixValue::Bool(true))));
        assert!(matches!(result2, Ok(NixValue::Bool(false))));

        assert!(!matches!(result1, Ok(NixValue::Bool(false))));
        assert!(!matches!(result2, Ok(NixValue::Bool(true))));

        assert!(!matches!(result1, Ok(NixValue::AttrSet(_))));
        assert!(!matches!(result2, Ok(NixValue::AttrSet(_))));
    }
    #[test]
    fn test_engine_lexer_primitives_parse_operator_add() {
        let mut data1 = setup_lexer("+");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Add));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_add() {
        let mut data1 = setup_lexer("test + test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Add, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_concat() {
        let mut data1 = setup_lexer("++");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Concat));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_concat() {
        let mut data1 = setup_lexer("test ++ test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Concat, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_sub() {
        let mut data1 = setup_lexer("-");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Sub));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_sub() {
        let mut data1 = setup_lexer("test - test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Sub, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_equal() {
        let mut data1 = setup_lexer("==");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Equal));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_equal() {
        let mut data1 = setup_lexer("test == test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Equal, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_unequal() {
        let mut data1 = setup_lexer("!=");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Unequal));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_unequal() {
        let mut data1 = setup_lexer("test != test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Unequal, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_and() {
        let mut data1 = setup_lexer("&&");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::And));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_and() {
        let mut data1 = setup_lexer("test && test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::And, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_merge() {
        let mut data1 = setup_lexer("//");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Merge));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_merge() {
        let mut data1 = setup_lexer("test // test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Merge, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_operator_divide() {
        let mut data1 = setup_lexer("/");
        let result1 = data1.parse_operator();

        assert!(result1.is_some());
        assert_eq!(result1, Some(Operator::Divide));
    }

    #[test]
    fn test_engine_lexer_primitves_parse_expression_divide() {
        let mut data1 = setup_lexer("test / test");
        let result1 = data1.parse_expression();
        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::BinaryOp { left: _, operator: Operator::Divide, right: _ })));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_indented_string() {
        let mut data1 = setup_lexer("test'';");
        let mut data2 = setup_lexer("${test}'';");
        let mut data3 = setup_lexer("test${test}test'';");
        let mut data4 = setup_lexer("test$(test)test'';");
        let mut data5 = setup_lexer("test''$(test)test'';");
        let mut data6 = setup_lexer("test''");
        let mut data7 = setup_lexer("test';");

        let result1 = data1.parse_indented_string();
        let result2 = data2.parse_indented_string();
        let result3 = data3.parse_indented_string();
        let result4 = data4.parse_indented_string();
        let result5 = data5.parse_indented_string();
        let result6 = data6.parse_indented_string();
        let result7 = data7.parse_indented_string();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        assert!(result4.is_ok());
        assert!(result5.is_ok());
        assert!(result6.is_err());
        assert!(result7.is_err());

        assert!(matches!(result1, Ok(NixValue::IndStr(_))));
        assert!(matches!(result2, Ok(NixValue::IndStr(_))));
        assert!(matches!(result3, Ok(NixValue::IndStr(_))));
        assert!(matches!(result4, Ok(NixValue::IndStr(_))));
        assert!(matches!(result5, Ok(NixValue::IndStr(_))));
    }

    #[test]
    fn test_engine_lexer_primitives_parse_application() {
        let mut data1 = setup_lexer("test test");
        let result1 = data1.parse_application();

        assert!(result1.is_ok());
        assert!(matches!(result1, Ok(NixValue::Apply(..))));
    }
}
