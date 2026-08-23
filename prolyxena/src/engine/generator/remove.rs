use std::path::PathBuf;

use crate::engine::generator::generate::IntoNixValue;
use crate::engine::core::*;
use crate::engine::formater::flattening::*;
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
                    Some(_) => { },
                    None => return Err("Remove-Fehler: Option ist nicht in Datei enthalten".to_string())
                };
            }
            NixValue::LetIn(map, body) => {
                match map.shift_remove(key) {
                    Some(_) => { },
                    None => return Err("Remove-Fehler: Option ist nicht in Datei enthalten".to_string())
                };
                match &mut **body {
                    NixValue::AttrSet(map) => {
                        match map.shift_remove(key) {
                            Some(_) => { },
                            None => return Err("Remove-Fehler: Option ist nicht in Datei enthalten".to_string())
                        }
                    }
                    NixValue::List(vec) => {
                        match value.into_nix()? {
                            Some(v) => {
                                vec.retain(|e| *e != v);
                            }
                            None => {
                                return Err("Remove-Fehler: Kann kein Element aus einer Liste entfernen, ohne Wert".to_string());
                            }
                        }
                    }
                    _ => {
                        return Err("Fehler: Der Zeil-Knoten muss ein Attribute Set oder Let In Statment sein".to_string());
                    }
                }

            }
            NixValue::List(vec) => {
                match value.into_nix()? {
                    Some(v) => {
                        vec.retain(|e| *e != v);
                    }
                    None => {
                        return Err("Remove-Fehler: Kann kein Element aus einer Liste entfernen, ohne Wert".to_string());
                    }
                }
            }
            _ => { }
        }

        self.expand();
        Ok(())
    }
}

impl Delete for FsData {
    fn delete_file(&mut self, name: &str) -> Result<(), String> {
        let full_path = PathBuf::from(&self.path).join(name.trim_start_matches('/')).display().to_string();
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
                return Err("Query-Fehler: Konnte den Dateinamen nicht extrahieren: hat letztes Segment schon extrahiert".to_string())
            }
        };
        if let FsNodes::Dir(map) = pointer {
            match map.shift_remove(*file_name) {
                Some(_) => Ok(()),
                None => return Err("Remove-Fehler: Ordner enthält die Datei nicht".to_string()),
            }
        } else {
            Err("Query-Fehler: Letzter Ordner nicht vorhanden".to_string())
        }
    }
}
