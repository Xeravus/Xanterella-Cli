use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum NixValue {
    AttrSet(HashMap<String, NixValue>),
    List(Vec<NixValue>),
    Str(String),
    Int(u64),
    Float(f64),
    Bool(bool),
    Identifier(String),
    LetIn(HashMap<String, NixValue>, Box<NixValue>),
    With(Box<NixValue>, Box<NixValue>),
    Lambda(Vec<String>, Option<String>, Box<NixValue>),
    Apply(Box<NixValue>, Box<NixValue>),
    Path(String),
}

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Parser {
            chars: content.chars().peekable(),
        }
    }

    pub fn parse_single_value(&mut self) -> Result<NixValue, String> {
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

    pub fn parse_path(&mut self) -> Result<NixValue, String> {
        let mut string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() || c == ';' {
                self.chars.next();
                break;
            }
            string.push(c);
            self.chars.next();
        }
        Ok(NixValue::Path(string))
    }

    pub fn parse_value(&mut self) -> Result<NixValue, String> {
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

    pub fn skip_whitespace(&mut self) {
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

    /*
    - - - - - Parse-Funktionen - - - - -
    */

    pub fn parse_attr_set(&mut self) -> Result<NixValue, String> {
        self.chars.next();
        let mut map = HashMap::new();
        loop {
            self.skip_whitespace();
            if let Some(&'}') = self.chars.peek() {
                self.chars.next();
                break;
            }

            let mut key = String::new();
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() || c == '=' {
                    break;
                }
                key.push(c);
                self.chars.next();
            }

            if key.is_empty() {
                return Err("Syntax-Fehler: Leerer Key im AttrSet".to_string());
            }

            if key == "inherit" {
                loop {
                    self.skip_whitespace();
                    if let Some(&';') = self.chars.peek() {
                        self.chars.next();
                        break;
                    }

                    let mut inherit = String::new();
                    while let Some(&c) = self.chars.peek() {
                        if c.is_whitespace() || c == ';' {
                            break;
                        }
                        inherit.push(c);
                        self.chars.next();
                    }
                    if inherit.is_empty() {
                        return Err("Syntax-Fehler: Unerwartetes Zeichen im 'inherit' Statment".to_string());
                    }
                    map.insert(
                        inherit.clone(),
                        NixValue::Identifier(inherit)
                    );
                }
                continue;
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
                return Err(format!("Syntax-Fehler: Erwartetes ';' nach dem Wert von '{}'", key));
            }
            map.insert(key, value);
        }
        Ok(NixValue::AttrSet(map))
    }

    pub fn parse_list(&mut self) -> Result<NixValue, String> {
        self.chars.next();
        let mut output_vec: Vec<NixValue> = vec![];
        loop {
            self.skip_whitespace();
            if let Some(&']') = self.chars.peek() {
                self.chars.next();
                break;
            }

            let value = self.parse_value()?;
            output_vec.push(value);
        }
        Ok(NixValue::List(output_vec))
    }

    pub fn parse_string(&mut self) -> Result<NixValue, String> {
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
        Ok(NixValue::Str(value))
    }

    pub fn parse_number(&mut self) -> Result<NixValue, String> {
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
                Ok(float_val) => Ok(NixValue::Float(float_val)),
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Kommazahl: '{}'", value_str)),
            }
        } else {
            match value_str.parse::<u64>() {
                Ok(int_val) => Ok(NixValue::Int(int_val)),
                Err(_) => Err(format!("Syntax-Fehler: Ungültige Ganzzahl '{}'", value_str)),
            }
        }
    }

    pub fn parse_identifier(&mut self) -> Result<NixValue, String> {
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
            "true" => Ok(NixValue::Bool(true)),
            "false" => Ok(NixValue::Bool(false)),
            _ => Ok(NixValue::Identifier(word)),
        }
    }

    pub fn parse_let_in(&mut self) ->  Result<NixValue, String> {
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

    pub fn parse_with(&mut self) -> Result<NixValue, String> {
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

    pub fn is_lambda_ahead(&self) -> bool {
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

    pub fn parse_lambda(&mut self) -> Result<NixValue, String> {
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

#[cfg(test)]
mod tests {
    // Importiert alles (Parser, NixValue) aus deiner eigentlichen Datei in das Test-Modul
    use super::*; 

    #[test]
    fn test_einfaches_attr_set() {
        let code = "{ boot = \"grub\"; enable = true; }";
        let mut parser = Parser::new(code);
        let ast = parser.parse_value().unwrap();
        match ast {
            NixValue::AttrSet(map) => {
                assert!(map.contains_key("boot"), "Der Key 'boot' fehlt!");
                assert!(map.contains_key("enable"), "Der Key 'enable' fehlt!");
            }
            _ => panic!("Der Parser hat kein AttrSet erkannt, sondern: {:#?}", ast),
        }
    }

    #[test]
    fn test_lambda_funktion() {
        let code = "{ pkgs, config }: { boot = \"grub\"; }";
        let mut parser = Parser::new(code);
        let ast = parser.parse_value().unwrap();
        match ast {
            NixValue::Lambda(args, body) => {
                assert_eq!(args.len(), 2, "Es sollten genau 2 Argumente sein");
                assert_eq!(args[0], "pkgs");
                assert_eq!(args[1], "config");
                assert!(matches!(*body, NixValue::AttrSet(_)), "Body der Funktion ist kein AttrSet!");
            }
            _ => panic!("Der Parser hat kein Lambda erkannt, sondern: {:#?}", ast),
        }
    }

    #[test]
    fn test_let_in_block() {
        let code = "let hostname = \"nixos\"; in { name = hostname; }";
        let mut parser = Parser::new(code);
        let ast = parser.parse_value().unwrap();

        match ast {
            NixValue::LetIn(map, body) => {
                assert!(map.contains_key("hostname"), "Die Zuweisung 'hostname' fehlt!");
                assert!(matches!(*body, NixValue::AttrSet(_)), "Body des let-in ist kein AttrSet!");
            }
            _ => panic!("Der Parser hat kein LetIn erkannt, sondern: {:#?}", ast),
        }
    }
}
