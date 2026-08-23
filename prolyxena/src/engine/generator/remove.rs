use crate::engine::generator::generate::IntoNixValue;
use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::formater::flattening::*;
use crate::engine::lexer::vfs::*;

pub trait Remove {
    fn remove<I: IntoNixValue>(&mut self, key: &str, value: I) -> Result<(), String>;
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
