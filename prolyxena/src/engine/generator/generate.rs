use indexmap::IndexMap;

use crate::engine::core::*;
use crate::engine::formater::flattening::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::vfs::*;

pub trait Generate {
    fn insert_from_string(&mut self, insert: &str) -> Result<(), String>;
}

pub trait Modify {
    fn generate_file(&mut self, name: &str, input: Option<NixValue>) -> Result<(), String>;
}

impl Generate for NixValue {
    fn insert_from_string(&mut self, insert: &str) -> Result<(), String> {
        let parts: Vec<&str> = insert.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err("Syntax-Fehler: Zuweisung muss ein '=' enthalten".to_string());
        }

        let key = parts[0].trim().to_string();
        let value = parts[1].trim().trim_end_matches(';');

        let mut lexer = Lexer::new(value, String::from("insert.nix"));
        let parsed_value = lexer.parse_single_value()?;

        self.flatten();

        match self {
            NixValue::AttrSet(map) => {
                map.insert(key, parsed_value);
            }
            NixValue::LetIn(map, _body) => {
                map.insert(key, parsed_value);
            }
            _ => {
                return Err("Fehler: Der Zeil-Knoten muss ein Attribute Set oder Let In Statment sein".to_string());
            }
        }
        self.expand();
        Ok(())
    }
}

impl Modify for FsData {
    fn generate_file(&mut self, name: &str, input: Option<NixValue>) -> Result<(), String> {
        let clean_path = name.trim_start_matches('/');
        let parts: Vec<&str> = clean_path.split('/').collect();

        let mut pointer = &mut self.fsnodes;
        #[allow(clippy::needless_range_loop)]
        for j in 0..parts.len() - 1 {
            let folder = parts[j];
            if let FsNodes::Dir(map) = pointer {
                pointer = map.entry(folder.to_string()).or_insert_with(|| FsNodes::Dir(IndexMap::new()));
            } else {
                return Err("Fehler: Versucht einen Ordner in einer Datei zu erstellen".to_string());
            };
        }
        let file_name = match parts.last() {
            Some(name) => name,
            None => return Err("Fehler: Konnte den letzten Namen nicht extrahieren: {}".to_string()),
        };
        if let FsNodes::Dir(map) = pointer {
            let ast = match input {
                Some(inp) => inp,
                None => NixValue::AttrSet(IndexMap::new()),
            };
            map.insert(file_name.to_string(), FsNodes::File { name: file_name.to_string(), ast });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
