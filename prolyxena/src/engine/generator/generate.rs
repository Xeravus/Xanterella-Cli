use indexmap::IndexMap;

use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::formater::flattening::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::vfs::*;

use std::path::PathBuf;

pub trait Generate {
    fn insert<I: IntoNixValue>(&mut self, key: &str, value: I) -> Result<(), String>;
}

pub trait Modify {
    fn generate_file<I: IntoNixValue>(&mut self, name: &str, input: I) -> Result<(), String>;
}

impl Generate for NixValue {
    fn insert<I: IntoNixValue>(&mut self, key: &str, value: I) -> Result<(), String> {
        let parsed_value = match value.into_nix()? {
            Some(v) => v,
            None => return Err("Fehler: Leerer Wert kann nicht eingefügt werden".to_string()),
        };
        self.flatten();

        match self {
            NixValue::AttrSet(map) => {
                map.insert(key.to_string(), parsed_value);
            }
            NixValue::LetIn(map, _body) => {
                map.insert(key.to_string(), parsed_value);
            }
            NixValue::List(vec) => {
                vec.push(parsed_value);
            }
            _ => {
                return Err("Fehler: Der Zeil-Knoten muss ein Attribute Set oder Let In Statment sein".to_string());
            }
        }
        self.expand();
        Ok(())
    }
}

pub trait IntoNixValue {
    fn into_nix(self) -> Result<Option<NixValue>, String>;
}

impl IntoNixValue for Option<NixValue> {
    fn into_nix(self) -> Result<Option<NixValue>, String> {
        Ok(self)
    }
}

impl IntoNixValue for NixValue {
    fn into_nix(self) -> Result<Option<NixValue>, String> {
        Ok(Some(self))
    }
}

impl IntoNixValue for &str {
    fn into_nix(self) -> Result<Option<NixValue>, String> {
        if self.trim().is_empty() {
            return Ok(None);
        }

        let mut lexer = Lexer::new(self, String::from(""));
        let parsed_value = lexer.parse_value()?;
        Ok(Some(parsed_value))
    }
}

impl IntoNixValue for String {
    fn into_nix(self) -> Result<Option<NixValue>, String> {
        self.as_str().into_nix()
    }
}

impl Modify for FsData {
    fn generate_file<I: IntoNixValue>(&mut self, name: &str, input: I) -> Result<(), String> {
        let full_path = PathBuf::from(&self.path).join(name.trim_start_matches('/')).display().to_string();
        let clean_path = full_path.trim_start_matches('/');
        let parts: Vec<&str> = clean_path.split('/').collect();
        let ast = match input.into_nix()? {
            Some(inp) => inp,
            None => NixValue::AttrSet(IndexMap::new()),
        };

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
            map.insert(file_name.to_string(), FsNodes::File { name: file_name.to_string(), ast });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_generator_insert_attrset_with_nixvalue() {
        let mut ast = NixValue::AttrSet(IndexMap::new());
        let result = ast.insert("enable", NixValue::Bool(true));
        
        assert!(result.is_ok());
        assert_eq!(
            ast,
            NixValue::AttrSet(IndexMap::from([("enable".to_string(), NixValue::Bool(true))]))
        );
    }

    #[test]
    fn test_engine_generator_insert_attrset_with_string() {
        let mut ast = NixValue::AttrSet(IndexMap::new());
        let result = ast.insert("port", "8080");
        
        assert!(result.is_ok());
        assert_eq!(
            ast,
            NixValue::AttrSet(IndexMap::from([("port".to_string(), NixValue::Int(8080))]))
        );
    }

    #[test]
    fn test_engine_generator_insert_nested_key_triggers_flatten_expand() {
        let mut ast = NixValue::AttrSet(IndexMap::new());
        let result = ast.insert("services.caddy.enable", "true");
        
        assert!(result.is_ok());
        
        let expected_ast = NixValue::AttrSet(IndexMap::from([(
            "services".to_string(),
            NixValue::AttrSet(IndexMap::from([(
                "caddy".to_string(),
                NixValue::AttrSet(IndexMap::from([(
                    "enable".to_string(),
                    NixValue::Bool(true),
                )])),
            )])),
        )]));
        
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_engine_generator_insert_into_list() {
        let mut ast = NixValue::List(vec![NixValue::Identifier("git".to_string())]);
        let result = ast.insert("", "\"nodejs\"");
        
        assert!(result.is_ok());
        assert_eq!(
            ast,
            NixValue::List(vec![
                NixValue::Identifier("git".to_string()),
                NixValue::Str("nodejs".to_string()),
            ])
        );
    }

    #[test]
    fn test_engine_generator_insert_into_let_in() {
        let mut ast = NixValue::LetIn(
            IndexMap::new(),
            Box::new(NixValue::Identifier("body".to_string()))
        );
        let result = ast.insert("var", "42");
        
        assert!(result.is_ok());
        assert_eq!(
            ast,
            NixValue::LetIn(
                IndexMap::from([("var".to_string(), NixValue::Int(42))]),
                Box::new(NixValue::Identifier("body".to_string()))
            )
        );
    }

    #[test]
    fn test_engine_generator_insert_invalid_target_node() {
        // Ein String ist kein valider Ziel-Knoten für ein Insert
        let mut ast = NixValue::Str("hello".to_string());
        let result = ast.insert("key", "true");
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Fehler: Der Zeil-Knoten muss ein Attribute Set oder Let In Statment sein"
        );
    }

    #[test]
    fn test_engine_generator_insert_invalid_string_syntax() {
        let mut ast = NixValue::AttrSet(IndexMap::new());
        let result = ast.insert("key", "{ kaputt");
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Syntax-Fehler"));
    }

    #[test]
    fn test_engine_generator_insert_empty_string() {
        let mut ast = NixValue::AttrSet(IndexMap::new());
        let result = ast.insert("key", "   ");
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Fehler: Leerer Wert kann nicht eingefügt werden"
        );
    }

    #[test]
    fn test_engine_generator_generate_generate_file_empty_tree() {
        let mut fsdata = FsData::new("/home/cato");
        fsdata.generate_file("file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: NixValue::AttrSet(IndexMap::new()) })])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_handle_input() {
        let mut fsdata = FsData::new("/home/cato");
        fsdata.generate_file("/file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: NixValue::AttrSet(IndexMap::new()) })])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_polymorphisim_none() {
        let mut fsdata = FsData::new("/home/cato");
        fsdata.generate_file("file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: NixValue::AttrSet(IndexMap::new()) })])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_polymorphisim_option_nixvalue() {
        let mut fsdata = FsData::new("/home/cato");
        let nixvalue = NixValue::AttrSet(IndexMap::from([(String::from("a"), NixValue::Identifier(String::from("b")))]));
        fsdata.generate_file("file.nix", Some(nixvalue.clone())).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: nixvalue})])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_polymorphisim_nixvalue() {
        let mut fsdata = FsData::new("/home/cato");
        let nixvalue = NixValue::AttrSet(IndexMap::from([(String::from("a"), NixValue::Identifier(String::from("b")))]));
        fsdata.generate_file("file.nix", nixvalue.clone()).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: nixvalue})])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_polymorphisim_string() {
        let mut fsdata = FsData::new("/home/cato");
        let nixvalue = NixValue::AttrSet(IndexMap::from([(String::from("a"), NixValue::Identifier(String::from("b")))]));
        let nixvalue_as_string = String::from("{ a = b; }");
        fsdata.generate_file("file.nix", nixvalue_as_string).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: nixvalue})])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_polymorphisim_str() {
        let mut fsdata = FsData::new("/home/cato");
        let nixvalue = NixValue::AttrSet(IndexMap::from([(String::from("a"), NixValue::Identifier(String::from("b")))]));
        let nixvalue_as_string = "{ a = b; }";
        fsdata.generate_file("file.nix", nixvalue_as_string).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: nixvalue})])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_filling_tree_with_operation() {
        let mut fsdata = FsData::new("/home/cato");

        // Step 1
        fsdata.generate_file("file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([(String::from("file.nix"), FsNodes::File { name: String::from("file.nix"), ast: NixValue::AttrSet(IndexMap::new()) })])
        ))])))])));
        // Step 2
        fsdata.generate_file("test_file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([
                                (String::from("file.nix"), FsNodes::File { name: String::from("file.nix"), ast: NixValue::AttrSet(IndexMap::new()) }),
                                (String::from("test_file.nix"), FsNodes::File { name: String::from("test_file.nix"), ast: NixValue::AttrSet(IndexMap::new()) })
                            ])
        ))])))])));
        // Step 3
        fsdata.generate_file("test/file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([
                                (String::from("file.nix"), FsNodes::File { name: String::from("file.nix"), ast: NixValue::AttrSet(IndexMap::new()) }),
                                (String::from("test_file.nix"), FsNodes::File { name: String::from("test_file.nix"), ast: NixValue::AttrSet(IndexMap::new()) }),
                                (String::from("test"), FsNodes::Dir(
                                        IndexMap::from([
                                            (String::from("file.nix"), FsNodes::File { name: String::from("file.nix"), ast: NixValue::AttrSet(IndexMap::new()) })
        ])))])))])))])));
    }
}
