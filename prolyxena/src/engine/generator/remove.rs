use std::path::PathBuf;

use crate::engine::core::*;
use crate::engine::formater::flattening::*;
use crate::engine::generator::generate::IntoNixValue;
use crate::engine::lexer::vfs::*;

pub trait Remove {
    fn remove<I: IntoNixValue>(&mut self, key: &str, value: I) -> Result<(), String>;
}

pub trait Delete {
    fn delete_file(&mut self, name: &str) -> Result<(), String>;
}

impl Remove for NixValue {
    fn remove<I: IntoNixValue>(&mut self, key: &str, value: I) -> Result<(), String> {
        self.flatten();

        match self {
            NixValue::AttrSet(map) => {
                match map.shift_remove(key) {
                    Some(_) => {}
                    None => return Err("Remove-Fehler: Option ist nicht in Datei enthalten".to_string()),
                };
            }
            NixValue::LetIn(map, body) => {
                match map.shift_remove(key) {
                    Some(_) => {}
                    None => return Err("Remove-Fehler: Option ist nicht in Datei enthalten".to_string()),
                };
                match &mut **body {
                    NixValue::AttrSet(map) => match map.shift_remove(key) {
                        Some(_) => {}
                        None => return Err("Remove-Fehler: Option ist nicht in Datei enthalten".to_string()),
                    },
                    NixValue::List(vec) => match value.into_nix()? {
                        Some(v) => {
                            vec.retain(|e| *e != v);
                        }
                        None => {
                            return Err(
                                "Remove-Fehler: Kann kein Element aus einer Liste entfernen, ohne Wert".to_string()
                            );
                        }
                    },
                    _ => {
                        return Err(
                            "Fehler: Der Zeil-Knoten muss ein Attribute Set oder Let In Statment sein".to_string()
                        );
                    }
                }
            }
            NixValue::List(vec) => match value.into_nix()? {
                Some(v) => {
                    vec.retain(|e| *e != v);
                }
                None => {
                    return Err("Remove-Fehler: Kann kein Element aus einer Liste entfernen, ohne Wert".to_string());
                }
            },
            _ => {}
        }

        self.expand();
        Ok(())
    }
}

impl Delete for FsData {
    fn delete_file(&mut self, name: &str) -> Result<(), String> {
        let full_path = PathBuf::from(name.trim_start_matches('/')).display().to_string();
        let clean_path = full_path.trim_start_matches('/');
        let parts: Vec<&str> = clean_path.split('/').collect();

        let mut pointer = &mut self.fsnodes;
        #[allow(clippy::needless_range_loop)]
        for j in 0..parts.len() - 1 {
            let folder = parts[j];
            if let FsNodes::Dir(map) = pointer {
                if let Some(child) = map.get_mut(folder) {
                    pointer = child;
                } else {
                    return Err(format!("Query-Fehler: Ordner '{}' nicht gefunden", folder));
                }
            } else {
                return Err("Query-Fehler: Versucht eine Ordner in einer Datei zu finden".to_string());
            }
        }
        let file_name = match parts.last() {
            Some(name) => name,
            None => {
                return Err(
                    "Query-Fehler: Konnte den Dateinamen nicht extrahieren: hat letztes Segment schon extrahiert"
                        .to_string(),
                );
            }
        };
        if let FsNodes::Dir(map) = pointer {
            match map.shift_remove(*file_name) {
                Some(_) => Ok(()),
                None => Err("Remove-Fehler: Ordner enthält die Datei nicht".to_string()),
            }
        } else {
            Err("Query-Fehler: Letzter Ordner nicht vorhanden".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::*;

    #[test]
    fn test_remove_attrset_success() {
        let mut ast = NixValue::AttrSet(IndexMap::from([("remove_me".to_string(), NixValue::Bool(true))]));

        let result = ast.remove("remove_me", None);

        assert!(result.is_ok());
        if let NixValue::AttrSet(map) = ast {
            assert!(map.is_empty());
        } else {
            panic!("Erwartet AttrSet");
        }
    }

    #[test]
    fn test_remove_attrset_not_found() {
        let mut ast = NixValue::AttrSet(IndexMap::new());
        let result = ast.remove("missing_key", None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Remove-Fehler: Option ist nicht in Datei enthalten");
    }

    #[test]
    fn test_remove_list_success() {
        let mut ast =
            NixValue::List(vec![NixValue::Identifier("keep".to_string()), NixValue::Identifier("drop".to_string())]);
        let result = ast.remove("", "drop");

        assert!(result.is_ok());
        if let NixValue::List(vec) = ast {
            assert_eq!(vec.len(), 1);
            assert_eq!(vec[0], NixValue::Identifier("keep".to_string()));
        } else {
            panic!("Erwartet List");
        }
    }

    #[test]
    fn test_remove_list_empty_value_error() {
        let mut ast = NixValue::List(vec![NixValue::Identifier("keep".to_string())]);
        let result = ast.remove("", None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Remove-Fehler: Kann kein Element aus einer Liste entfernen, ohne Wert");
    }

    #[test]
    fn test_remove_letin_success() {
        let map = IndexMap::from([("target".to_string(), NixValue::Int(1))]);
        let body = NixValue::AttrSet(IndexMap::from([("target".to_string(), NixValue::Int(2))]));
        let mut ast = NixValue::LetIn(map, Box::new(body));

        let result = ast.remove("target", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_file_success() {
        let mut fs_data = FsData::new("/fake/root");

        let file_map = IndexMap::from([(
            "config.nix".to_string(),
            FsNodes::File { name: "config.nix".to_string(), ast: NixValue::Bool(true) },
        )]);

        let folder_map = IndexMap::from([("node1".to_string(), FsNodes::Dir(file_map))]);

        fs_data.fsnodes = FsNodes::Dir(folder_map);

        let result = fs_data.delete_file("node1/config.nix");
        assert!(result.is_ok());
        if let FsNodes::Dir(root_map) = &fs_data.fsnodes {
            if let FsNodes::Dir(node1_map) = &root_map["node1"] {
                assert!(node1_map.is_empty());
            } else {
                panic!("node1 sollte ein Ordner sein");
            }
        } else {
            panic!("Root sollte ein Ordner sein");
        }
    }

    #[test]
    fn test_delete_file_not_found() {
        let mut fs_data = FsData::new("/fake/root");
        fs_data.fsnodes = FsNodes::Dir(IndexMap::from([("node1".to_string(), FsNodes::Dir(IndexMap::new()))]));

        let result = fs_data.delete_file("node1/missing.nix");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Remove-Fehler: Ordner enthält die Datei nicht");
    }

    #[test]
    fn test_delete_file_invalid_path() {
        let mut fs_data = FsData::new("/fake/root");
        fs_data.fsnodes = FsNodes::Dir(IndexMap::from([(
            "file_as_dir.nix".to_string(),
            FsNodes::File { name: "file_as_dir.nix".to_string(), ast: NixValue::Bool(true) },
        )]));

        let result = fs_data.delete_file("file_as_dir.nix/config.nix");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Letzter Ordner nicht vorhanden");
    }

    #[test]
    fn test_remove_letin_body_list_success() {
        let map = IndexMap::from([("target_key".to_string(), NixValue::Int(1))]);
        let body = NixValue::List(vec![
            NixValue::Identifier("keep_me".to_string()),
            NixValue::Identifier("drop_me".to_string()),
        ]);
        let mut ast = NixValue::LetIn(map, Box::new(body));

        let result = ast.remove("target_key", "drop_me");

        assert!(result.is_ok());
        if let NixValue::LetIn(new_map, new_body) = ast {
            assert!(new_map.is_empty());
            if let NixValue::List(vec) = *new_body {
                assert_eq!(vec.len(), 1);
                assert_eq!(vec[0], NixValue::Identifier("keep_me".to_string()));
            } else {
                panic!("Erwartet List im Body");
            }
        } else {
            panic!("Erwartet LetIn");
        }
    }

    #[test]
    fn test_remove_letin_body_list_empty_value_error() {
        let map = IndexMap::from([("target_key".to_string(), NixValue::Int(1))]);
        let body = NixValue::List(vec![NixValue::Identifier("keep_me".to_string())]);
        let mut ast = NixValue::LetIn(map, Box::new(body));
        let result = ast.remove("target_key", None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Remove-Fehler: Kann kein Element aus einer Liste entfernen, ohne Wert");
    }

    #[test]
    fn test_remove_letin_body_invalid_type() {
        let map = IndexMap::from([("target_key".to_string(), NixValue::Int(1))]);
        let body = NixValue::Int(42); // Weder AttrSet noch List
        let mut ast = NixValue::LetIn(map, Box::new(body));

        let result = ast.remove("target_key", None);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Fehler: Der Zeil-Knoten muss ein Attribute Set oder Let In Statment sein");
    }

    #[test]
    fn test_remove_letin_body_attrset_success() {
        let map = IndexMap::from([("target_key".to_string(), NixValue::Int(1))]);
        let body = NixValue::AttrSet(IndexMap::from([("target_key".to_string(), NixValue::Int(2))]));
        let mut ast = NixValue::LetIn(map, Box::new(body));

        let result = ast.remove("target_key", None);

        assert!(result.is_ok());
        if let NixValue::LetIn(new_map, new_body) = ast {
            assert!(new_map.is_empty());
            if let NixValue::AttrSet(body_map) = *new_body {
                assert!(body_map.is_empty());
            } else {
                panic!("Erwartet AttrSet im Body");
            }
        } else {
            panic!("Erwartet LetIn");
        }
    }
}
