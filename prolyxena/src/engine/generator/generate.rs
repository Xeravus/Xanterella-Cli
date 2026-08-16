use indexmap::IndexMap;

use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::formater::flattening::*;
use crate::engine::lexer::primitives::*;
use crate::engine::lexer::vfs::*;

pub trait Generate {
    fn insert_from_string(&mut self, insert: &str) -> Result<(), String>;
}

pub trait Modify {
    fn generate_file<I: IntoNixValue>(&mut self, name: &str, input: I) -> Result<(), String>;
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
        let clean_path = name.trim_start_matches('/');
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
    fn test_engine_generator_generate_generate_file_empty_tree() {
        let mut fsdata = FsData::new("/home/cato");
        fsdata.generate_file("/home/cato/file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([("file.nix".to_string(), FsNodes::File { name: "file.nix".to_string(), ast: NixValue::AttrSet(IndexMap::new()) })])
        ))])))])));
    }

    #[test]
    fn test_engine_generator_generate_generate_file_polymorphisim_none() {
        let mut fsdata = FsData::new("/home/cato");
        fsdata.generate_file("/home/cato/file.nix", None).unwrap();

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
        fsdata.generate_file("/home/cato/file.nix", Some(nixvalue.clone())).unwrap();

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
        fsdata.generate_file("/home/cato/file.nix", nixvalue.clone()).unwrap();

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
        fsdata.generate_file("/home/cato/file.nix", nixvalue_as_string).unwrap();

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
        fsdata.generate_file("/home/cato/file.nix", nixvalue_as_string).unwrap();

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
        fsdata.generate_file("/home/cato/file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([(String::from("file.nix"), FsNodes::File { name: String::from("file.nix"), ast: NixValue::AttrSet(IndexMap::new()) })])
        ))])))])));
        // Step 2
        fsdata.generate_file("/home/cato/test_file.nix", None).unwrap();

        assert_eq!(fsdata.fsnodes, FsNodes::Dir(
                IndexMap::from([(String::from("home"), FsNodes::Dir(
                        IndexMap::from([(String::from("cato"), FsNodes::Dir(
                            IndexMap::from([
                                (String::from("file.nix"), FsNodes::File { name: String::from("file.nix"), ast: NixValue::AttrSet(IndexMap::new()) }),
                                (String::from("test_file.nix"), FsNodes::File { name: String::from("test_file.nix"), ast: NixValue::AttrSet(IndexMap::new()) })
                            ])
        ))])))])));
        // Step 3
        fsdata.generate_file("/home/cato/test/file.nix", None).unwrap();

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
