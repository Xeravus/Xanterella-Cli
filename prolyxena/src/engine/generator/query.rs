use crate::engine::core::*;
use crate::engine::lexer::vfs::*;

pub trait Query {
    fn query_exact_mut<'a>(&'a mut self, path: &[&str]) -> Vec<&'a mut NixValue>;
    fn query_exact_inner<'a>(&'a mut self, path: &[&str], result: &mut Vec<&'a mut NixValue>);

    fn query_fuzzy_mut<'a>(&'a mut self, term: &str) -> Vec<&'a mut NixValue>;
    fn query_fuzzy_inner<'a>(&'a mut self, term: &str, result: &mut Vec<&'a mut NixValue>);
}

pub trait SearchContent {
    fn search_tree<'a>(&'a mut self, name: &str) -> Result<&'a mut NixValue, String>;
}

pub trait SearchObjekt {
    fn search_dir(&self, name: &str) -> Result<Vec<String>, String>;
    fn search_dir_inner<'a>(&'a self, name: &str, path: &mut Vec<&'a str>, result: &mut Vec<String>);
    fn dir_list_files(&self, name: &str) -> Result<Vec<String>, String>;
    fn search_file(&self, name: &str) -> Result<Vec<String>, String>;
    fn search_file_inner<'a>(&'a self, name: &str, path: &mut Vec<&'a str>, result: &mut Vec<String>);
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

impl SearchContent for FsData {
    fn search_tree<'a>(&'a mut self, name: &str) -> Result<&'a mut NixValue, String> {
        let parts: Vec<&str> = name.trim_start_matches('/').split('/').filter(|s| !s.is_empty()).collect();
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
                return Err(
                    "Query-Fehler: Konnte den Dateinamen nicht extrahieren: hat letztes Segment schon extrahiert"
                        .to_string(),
                );
            }
        };
        if let FsNodes::Dir(map) = pointer {
            match map.get_mut(*file_name) {
                Some(FsNodes::File { ast, .. }) => Ok(ast),
                Some(FsNodes::Dir(_)) => Err("Query-Fehler: Gesuchte Datei stellt sich als Ordner heraus".to_string()),
                None => Err("Query-Fehler: Ordner enthält die Datei nicht".to_string()),
            }
        } else {
            Err("Query-Fehler: Versucht einen Ordner in einer Datei zu finden".to_string())
        }
    }
}

impl SearchObjekt for FsNodes {
    fn search_dir(&self, name: &str) -> Result<Vec<String>, String> {
        if name.is_empty() {
            return Err("Query-Fehler: Leere Query".to_string());
        }
        let mut result = Vec::new();
        let mut path = Vec::new();

        self.search_dir_inner(&name, &mut path, &mut result);

        Ok(result)
    }

    fn search_dir_inner<'a>(&'a self, name: &str, path: &mut Vec<&'a str>, result: &mut Vec<String>) {
        match self {
            FsNodes::Dir(map) => {
                if map.contains_key(name) {
                    let mut full_path = path.join("/");
                    if !full_path.is_empty() {
                        full_path.push('/');
                    }
                    full_path.push_str(name);
                    result.push(full_path);
                }
                for (child_name, child_node) in map {
                    path.push(&child_name);
                    child_node.search_dir_inner(name, path, result);
                    path.pop();
                }
            }
            _ => { }
        }
    }

    fn dir_list_files(&self, name: &str) -> Result<Vec<String>, String> {
        if name.is_empty() {
            return Err("Query-Fehler: Leere Query".to_string());
        }
        let parts: Vec<&str> = name.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
        let mut pointer = self;
        #[allow(clippy::needless_range_loop)]
        for i in 0..parts.len() {
            let folder = parts[i];
            if let FsNodes::Dir(map) = pointer {
                if let Some(child) = map.get(folder) {
                    pointer = child;
                } else {
                    return Err(format!("Query-Fehler: Ordner '{}' nicht gefunden", folder));
                }
            } else {
                return Err("Query-Fehler: Versucht einen Ordner in einer Datei zu finden".to_string());
            }
        }
        if let FsNodes::Dir(map) = pointer {
            Ok(map.keys().map(|f| f.to_string()).collect())
        } else {
            Err(format!("Query-Fehler: Ordner nicht gefunden"))
        }
    }

    fn search_file(&self, name: &str) -> Result<Vec<String>, String> {
        if name.is_empty() {
            return Err("Query-Fehler: Leere Query".to_string());
        }
        let mut result = Vec::new();
        let mut path = Vec::new();

        self.search_file_inner(&name, &mut path, &mut result);

        Ok(result)
    }

    fn search_file_inner<'a>(&'a self, name: &str, path: &mut Vec<&'a str>, result: &mut Vec<String>) {
        match self {
            FsNodes::Dir(map) => {
                if map.contains_key(name) {
                    let mut full_path = path.join("/");
                    if !full_path.is_empty() {
                        full_path.push('/');
                    }
                    full_path.push_str(name);
                    result.push(full_path);
                }
                for (child_name, child_node) in map {
                    path.push(&child_name);
                    child_node.search_dir_inner(name, path, result);
                    path.pop();
                }
            }
            FsNodes::File { name: file_name, .. } => {
                if name == file_name {
                    let mut full_path = path.join("/");
                    if !full_path.is_empty() {
                        full_path.push('/');
                    }
                    full_path.push_str(name);
                    result.push(full_path);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::*;

    #[test]
    fn test_engine_generator_query_query_exact_attrset() {
        let mut ast = NixValue::AttrSet(IndexMap::from([(
            "server".to_string(),
            NixValue::AttrSet(IndexMap::from([("port".to_string(), NixValue::Int(8080))])),
        )]));

        let result = ast.query_exact_mut(&["server", "port"]);
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0], NixValue::Int(8080));
    }

    #[test]
    fn test_engine_generator_query_query_exact_let_in() {
        let mut ast = NixValue::LetIn(
            IndexMap::new(),
            Box::from(NixValue::AttrSet(IndexMap::from([(
                "server".to_string(),
                NixValue::AttrSet(IndexMap::from([("port".to_string(), NixValue::Int(8080))])),
            )]))),
        );

        let result = ast.query_exact_mut(&["server", "port"]);
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0], NixValue::Int(8080));
    }

    #[test]
    fn test_engine_generator_query_query_exact_not_found() {
        let mut ast = NixValue::AttrSet(IndexMap::from([("server".to_string(), NixValue::Int(8080))]));

        let result = ast.query_exact_mut(&["server", "ip"]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_engine_generator_query_query_fuzzy_identifier() {
        let mut ast = NixValue::AttrSet(IndexMap::from([(
            "services".to_string(),
            NixValue::List(vec![
                NixValue::Identifier("my-custom-service".to_string()),
                NixValue::Identifier("other-service".to_string()),
            ]),
        )]));

        let result = ast.query_fuzzy_mut("custom");
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0], NixValue::Identifier("my-custom-service".to_string()));
    }

    #[test]
    fn test_engine_generator_query_search_tree_success() {
        let mut fs_data = FsData::new("/fake/root");
        let ast = NixValue::Bool(true);

        let file_map =
            IndexMap::from([("config.nix".to_string(), FsNodes::File { name: "config.nix".to_string(), ast })]);

        let folder_map = IndexMap::from([("node1".to_string(), FsNodes::Dir(file_map))]);

        fs_data.fsnodes = FsNodes::Dir(folder_map);

        let result = fs_data.search_tree("node1/config.nix");
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), NixValue::Bool(true));
    }

    #[test]
    fn test_engine_generator_query_search_tree_folder_not_found() {
        let mut fs_data = FsData::new("/fake/root");

        let result = fs_data.search_tree("node1/config.nix");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Ordner 'node1' nicht gefunden");
    }

    #[test]
    fn test_engine_generator_query_search_tree_file_not_found() {
        let mut fs_data = FsData::new("/fake/root");

        let folder_map = IndexMap::from([("node1".to_string(), FsNodes::Dir(IndexMap::new()))]);

        fs_data.fsnodes = FsNodes::Dir(folder_map);

        let result = fs_data.search_tree("node1/config.nix");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Ordner enthält die Datei nicht");
    }

    #[test]
    fn test_engine_generator_query_query_exact_letin_map() {
        let mut ast = NixValue::LetIn(
            IndexMap::from([("target_key".to_string(), NixValue::Int(42))]),
            Box::new(NixValue::Identifier("body".to_string())),
        );
        let result = ast.query_exact_mut(&["target_key"]);
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0], NixValue::Int(42));
    }

    #[test]
    fn test_query_exact_list() {
        let mut ast = NixValue::List(vec![
            NixValue::AttrSet(IndexMap::from([("key".to_string(), NixValue::Int(1))])),
            NixValue::AttrSet(IndexMap::from([("key".to_string(), NixValue::Int(2))])),
        ]);
        let result = ast.query_exact_mut(&["key"]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_engine_generator_query_query_exact_apply_and_group() {
        let mut ast = NixValue::Group(Box::new(NixValue::Apply(
            Box::new(NixValue::Identifier("func".to_string())),
            Box::new(NixValue::AttrSet(IndexMap::from([("target".to_string(), NixValue::Int(99))]))),
        )));
        let result = ast.query_exact_mut(&["target"]);
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0], NixValue::Int(99));
    }

    #[test]
    fn test_engine_generator_query_query_exact_catch_all() {
        let mut ast = NixValue::Int(42);
        let result = ast.query_exact_mut(&["missing"]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_engine_generator_query_query_fuzzy_attrset_key_match() {
        let mut ast = NixValue::AttrSet(IndexMap::from([("my-target-key".to_string(), NixValue::Int(100))]));
        let result = ast.query_fuzzy_mut("target");
        assert_eq!(result.len(), 1);
        assert_eq!(*result[0], NixValue::Int(100));
    }

    #[test]
    fn test_engine_generator_query_query_fuzzy_letin() {
        let mut ast = NixValue::LetIn(
            IndexMap::from([("let-target-key".to_string(), NixValue::Int(1))]),
            Box::new(NixValue::Identifier("target-body".to_string())),
        );
        let result = ast.query_fuzzy_mut("target");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_engine_generator_query_query_fuzzy_group_and_antiquotation() {
        let mut ast = NixValue::Group(Box::new(NixValue::Antiquotation(Box::new(NixValue::Identifier(
            "fuzzy-target".to_string(),
        )))));
        let result = ast.query_fuzzy_mut("fuzzy");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_engine_generator_query_query_fuzzy_binary_op_and_with() {
        let mut ast = NixValue::BinaryOp {
            left: Box::new(NixValue::Identifier("left-target".to_string())),
            operator: Operator::Add,
            right: Box::new(NixValue::With(
                Box::new(NixValue::Identifier("namespace".to_string())),
                Box::new(NixValue::Identifier("right-target".to_string())),
            )),
        };
        let result = ast.query_fuzzy_mut("target");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_engine_generator_query_query_fuzzy_catch_all() {
        let mut ast = NixValue::Bool(true);
        let result = ast.query_fuzzy_mut("target");
        assert!(result.is_empty());
    }

    #[test]
    fn test_search_tree_empty_query() {
        let mut fs_data = FsData::new("/fake/root");
        let result = fs_data.search_tree("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Leere Treequery");
    }

    #[test]
    fn test_search_tree_file_instead_of_folder() {
        let mut fs_data = FsData::new("/fake/root");
        let file_node = FsNodes::File { name: "file.nix".to_string(), ast: NixValue::Bool(true) };
        fs_data.fsnodes = FsNodes::Dir(IndexMap::from([("file.nix".to_string(), file_node)]));

        let result = fs_data.search_tree("file.nix/child.nix");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Versucht einen Ordner in einer Datei zu finden");
    }

    #[test]
    fn test_search_tree_target_is_directory() {
        let mut fs_data = FsData::new("/fake/root");
        let sub_folder = FsNodes::Dir(IndexMap::new());
        fs_data.fsnodes = FsNodes::Dir(IndexMap::from([("subfolder".to_string(), sub_folder)]));

        let result = fs_data.search_tree("subfolder");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Gesuchte Datei stellt sich als Ordner heraus");
    }

    #[test]
    fn test_fsnodes_search_dir_success() {
        let target_dir = FsNodes::Dir(IndexMap::new());
        let root = FsNodes::Dir(IndexMap::from([
            ("target".to_string(), target_dir.clone()),
            ("nested".to_string(), FsNodes::Dir(IndexMap::from([
                ("target".to_string(), target_dir)
            ]))),
        ]));

        let result = root.search_dir("target").unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"target".to_string()));
        assert!(result.contains(&"nested/target".to_string()));
    }

    #[test]
    fn test_fsnodes_search_dir_empty() {
        let root = FsNodes::Dir(IndexMap::new());
        assert!(root.search_dir("").is_err());
    }

    #[test]
    fn test_fsnodes_dir_list_files_success() {
        let folder = FsNodes::Dir(IndexMap::from([
            ("file1.nix".to_string(), FsNodes::File { name: "file1.nix".to_string(), ast: NixValue::Bool(true) }),
            ("file2.nix".to_string(), FsNodes::File { name: "file2.nix".to_string(), ast: NixValue::Bool(true) }),
        ]));
        let root = FsNodes::Dir(IndexMap::from([("myfolder".to_string(), folder)]));

        let result = root.dir_list_files("myfolder").unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"file1.nix".to_string()));
        assert!(result.contains(&"file2.nix".to_string()));
    }

    #[test]
    fn test_fsnodes_dir_list_files_not_found() {
        let root = FsNodes::Dir(IndexMap::new());
        assert!(root.dir_list_files("ghostfolder").is_err());
    }

    #[test]
    fn test_fsnodes_dir_list_files_is_file() {
        let root = FsNodes::Dir(IndexMap::from([
            ("myfile.nix".to_string(), FsNodes::File { name: "myfile.nix".to_string(), ast: NixValue::Bool(true) })
        ]));
        let result = root.dir_list_files("myfile.nix");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Query-Fehler: Ordner nicht gefunden");
    }

    #[test]
    fn test_fsnodes_search_file_success() {
        let target_file = FsNodes::File { name: "target.nix".to_string(), ast: NixValue::Bool(true) };
        let root = FsNodes::Dir(IndexMap::from([
            ("target.nix".to_string(), target_file.clone()),
            ("subfolder".to_string(), FsNodes::Dir(IndexMap::from([
                ("target.nix".to_string(), target_file)
            ]))),
        ]));

        let result = root.search_file("target.nix").unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"target.nix".to_string()));
        assert!(result.contains(&"subfolder/target.nix".to_string()));
    }

    #[test]
    fn test_fsnodes_search_file_empty() {
        let root = FsNodes::Dir(IndexMap::new());
        assert!(root.search_file("").is_err());
    }
}
