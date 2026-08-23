use crate::engine::core::*;
use crate::engine::lexer::vfs::*;

pub trait Query {
    fn query_exact_mut<'a>(&'a mut self, path: &[&str]) -> Vec<&'a mut NixValue>;
    fn query_exact_inner<'a>(&'a mut self, path: &[&str], result: &mut Vec<&'a mut NixValue>);

    fn query_fuzzy_mut<'a>(&'a mut self, term: &str) -> Vec<&'a mut NixValue>;
    fn query_fuzzy_inner<'a>(&'a mut self, term: &str, result: &mut Vec<&'a mut NixValue>);
}

pub trait Search {
    fn search_tree<'a>(&'a mut self, name: &str) -> Result<&'a mut NixValue, String>;
}

impl Query for NixValue {
    fn query_exact_mut<'a>(&'a mut self, path: &[&str]) -> Vec<&'a mut NixValue> {
        let mut result = Vec::new();
        self.query_exact_inner(path, &mut result);
        result
    }

    fn query_exact_inner<'a>(&'a mut self, path: &[&str], result: &mut Vec<&'a mut NixValue>) {
        if path.is_empty() {
            result.push(self);
            return;
        }

        match self {
            NixValue::AttrSet(map) => {
                if let Some(value) = map.get_mut(path[0]) {
                    value.query_exact_inner(&path[1..], result);
                }
            }
            NixValue::LetIn(map, body) => {
                if let Some(value) = map.get_mut(path[0]) {
                    value.query_exact_inner(&path[1..], result);
                }
                body.query_exact_inner(path, result);
            }
            NixValue::List(vec) => {
                for i in vec.iter_mut() {
                    i.query_exact_inner(path, result);
                }
            }
            NixValue::Apply(_, right) | NixValue::Group(right) => {
                right.query_exact_inner(path, result);
            }
            _ => {}
        }
    }

    fn query_fuzzy_mut<'a>(&'a mut self, term: &str) -> Vec<&'a mut NixValue> {
        let mut result = Vec::new();
        let term_lower = term.to_lowercase();
        self.query_fuzzy_inner(&term_lower, &mut result);
        result
    }

    fn query_fuzzy_inner<'a>(&'a mut self, term: &str, result: &mut Vec<&'a mut NixValue>) {
        let is_self_match = match self {
            NixValue::Identifier(id) => id.to_lowercase().contains(term),
            _ => false,
        };

        if is_self_match {
            result.push(self);
            return;
        }

        match self {
            NixValue::AttrSet(map) => {
                for (key, value) in map.iter_mut() {
                    if key.to_lowercase().contains(term) {
                        result.push(value);
                    } else {
                        value.query_fuzzy_inner(term, result);
                    }
                }
            }
            NixValue::LetIn(map, body) => {
                for (key, value) in map.iter_mut() {
                    if key.to_lowercase().contains(term) {
                        result.push(value);
                    } else {
                        value.query_fuzzy_inner(term, result);
                    }
                }
                body.query_fuzzy_inner(term, result);
            }
            NixValue::List(vec) => {
                for i in vec.iter_mut() {
                    i.query_fuzzy_inner(term, result);
                }
            }
            NixValue::Group(inner) | NixValue::Antiquotation(inner) => {
                inner.query_fuzzy_inner(term, result);
            }
            NixValue::Apply(left, right) | NixValue::With(left, right) | NixValue::BinaryOp { left, right, .. } => {
                left.query_fuzzy_inner(term, result);
                right.query_fuzzy_inner(term, result);
            }
            _ => {}
        }
    }
}

impl Search for FsData {
    fn search_tree<'a>(&'a mut self, name: &str) -> Result<&'a mut NixValue, String> {
        let parts: Vec<&str> = name.trim_start_matches('/').split('/').collect();
        if parts.is_empty() {
            return Err("Query-Fehler: Leere Treequery".to_string());
        }

        let mut pointer = &mut self.fsnodes;
        #[allow(clippy::needless_range_loop)]
        for i in 0..parts.len() - 1 {
            let folder = parts[i];
            if let FsNodes::Dir(map) = pointer {
                if let Some(child) = map.get_mut(folder) {
                    pointer = child;
                } else {
                    return Err(format!("Query-Fehler: Ordner '{}' nicht gefunden", folder));
                }
            } else {
                return Err("Query-Fehler: Versucht einen Ordner in einer Datei zu finden".to_string());
            }
        }
        let file_name = match parts.last() {
            Some(name) => name,
            None => {
                return Err("Query-Fehler: Konnte den Dateinamen nicht extrahieren: hat letztes Segment schon extrahiert".to_string())
            }
        };
        if let FsNodes::Dir(map) = pointer {
            match map.get_mut(*file_name) {
                Some(FsNodes::File { ast, .. }) => Ok(ast),
                Some(FsNodes::Dir(_)) => Err("Query-Fehler: Gesuchte Datei stellt sich als Ordner heraus".to_string()),
                None => Err("Query-Fehler: Ordner enthält die Datei nicht".to_string()),
            }
        } else {
            Err("Query-Fehler: Letzter Ordner nicht vorhanden".to_string())
        }
    }
}
