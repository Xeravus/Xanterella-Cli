use std::fs;
use std::process;
use std::sync::mpsc::Sender;
use std::time::Instant;

use indexmap::IndexMap;
use walkdir::WalkDir;

use crate::engine::core::*;
use crate::engine::formater::flattening::Flattening;
use crate::engine::formater::sort::Sort;
use crate::engine::lexer::core::*;

#[derive(Debug, Clone)]
pub struct FsData {
    pub files: Vec<String>,
    pub path: String,
    pub fsnodes: FsNodes,
    pub trans: Option<Sender<ParseEvent>>,
    pub time: f64,
    pub sort: bool,
    pub expand: bool,
    pub flatten: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FsNodes {
    File { name: String, ast: NixValue },
    Dir(IndexMap<String, FsNodes>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReOderMode {
    Expand,
    Flatten,
    Sort,
}

impl FsData {
    pub fn new(path: &str) -> Self {
        FsData {
            files: vec![],
            path: path.to_string(),
            fsnodes: FsNodes::Dir(IndexMap::new()),
            trans: None,
            time: 0.0,
            sort: false,
            expand: false,
            flatten: false,
        }
    }

    pub fn new_trans(path: &str, trans: Sender<ParseEvent>) -> Self {
        FsData {
            files: vec![],
            path: path.to_string(),
            fsnodes: FsNodes::Dir(IndexMap::new()),
            trans: Some(trans),
            time: 0.0,
            sort: false,
            expand: false,
            flatten: false,
        }
    }

    pub fn set_sort(&mut self, sort: bool) {
        self.sort = sort;
    }

    pub fn set_expand(&mut self, expand: bool) {
        self.expand = expand;
    }

    pub fn set_flatten(&mut self, flatten: bool) {
        self.flatten = flatten;
    }

    pub fn load(&mut self) -> Result<(), String> {
        let start = Instant::now();
        self.get_files();
        self.gen_tree()?;
        self.time = start.elapsed().as_secs_f64();
        if let Some(tx) = &self.trans {
            tx.send(ParseEvent::Finished(self.get_time())).ok();
        }
        Ok(())
    }

    pub fn get_files(&mut self) {
        if let Some(tx) = &self.trans {
            let _ = tx.send(ParseEvent::StartGettingFiles);
        }
        let files: Vec<String> = if !self.path.ends_with(".nix") {
            WalkDir::new(&self.path)
                .min_depth(1)
                .sort_by_file_name()
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter_map(|entry| {
                    let path_str = entry.path().to_str()?;
                    if path_str.ends_with(".nix") { Some(path_str.to_string()) } else { None }
                })
                .collect()
        } else if self.path.ends_with(".nix") {
            vec![self.path.to_string()]
        } else {
            eprintln!("Fehler: Keine Nix Datei(n) angegeben");
            process::exit(1);
        };
        if let Some(tx) = &self.trans {
            let _ = tx.send(ParseEvent::EndGettingFiles);
        }
        self.files = files;
    }

    pub fn gen_tree(&mut self) -> Result<(), String> {
        if let Some(tx) = &self.trans {
            let _ = tx.send(ParseEvent::StartGen);
        }
        let files = self.files.clone();
        for i in files {
            let rel_path = i.strip_prefix(&self.path).unwrap_or(&i);
            let clean_path = rel_path.trim_start_matches('/');
            let parts: Vec<&str> = clean_path.split('/').collect();
            if parts.is_empty() {
                continue;
            }

            let mut pointer = &mut self.fsnodes;
            #[allow(clippy::needless_range_loop)]
            for j in 0..parts.len() - 1 {
                let folder = parts[j];
                if let FsNodes::Dir(map) = pointer {
                    pointer = map.entry(folder.to_string()).or_insert_with(|| FsNodes::Dir(IndexMap::new()));
                } else {
                    return Err("Fehler: Versucht einen Ordner in einer Datei zu erstellen".to_string());
                }
            }
            let file_name = match parts.last() {
                Some(name) => name,
                None => {
                    return Err(
                        "Fehler: Konnte den letzten Namen nicht extrahieren: hat letztes segment schon extrahiert"
                            .to_string(),
                    );
                }
            };
            if let FsNodes::Dir(map) = pointer {
                let content = fs::read_to_string(&i).unwrap();
                if let Some(tx) = &self.trans {
                    let _ = tx.send(ParseEvent::StartParsingFile(clean_path.to_string()));
                }
                let mut file_data = match &self.trans {
                    Some(tx) => Lexer::new_trans(&content, i.clone(), tx.clone()),
                    None => Lexer::new(&content, i.clone()),
                };
                let ast = match file_data.parse_value() {
                    Ok(mut parsed_ast) => {
                        if self.sort || self.expand || self.flatten {
                            if let Some(tx) = &self.trans {
                                if self.expand {
                                    let _ = tx.send(ParseEvent::StartExpandingFile(clean_path.to_string()));
                                    parsed_ast.expand();
                                    let _ = tx.send(ParseEvent::EndExpandingFile(clean_path.to_string()));
                                }
                                if self.flatten {
                                    let _ = tx.send(ParseEvent::StartFlatteningFile(clean_path.to_string()));
                                    parsed_ast.flatten();
                                    let _ = tx.send(ParseEvent::EndFlatteningFile(clean_path.to_string()));
                                }
                                if self.sort {
                                    let _ = tx.send(ParseEvent::StartSortingFile(clean_path.to_string()));
                                    parsed_ast.sort_ast();
                                    let _ = tx.send(ParseEvent::EndSortingFile(clean_path.to_string()));
                                }
                            } else {
                                if self.expand {
                                    parsed_ast.expand();
                                }
                                if self.flatten {
                                    parsed_ast.flatten();
                                }
                                if self.sort {
                                    parsed_ast.sort_ast();
                                }
                            }
                        }
                        parsed_ast
                    }
                    Err(e) => {
                        return Err(format!("Fehler beim Parsen/generating des Trees: \n{}", e));
                    }
                };
                if let Some(tx) = &self.trans {
                    let _ = tx.send(ParseEvent::EndParsingFile(clean_path.to_string()));
                }
                map.insert(file_name.to_string(), FsNodes::File { name: file_name.to_string(), ast });
            }
        }
        if let Some(tx) = &self.trans {
            let _ = tx.send(ParseEvent::EndGen);
        }
        Ok(())
    }

    pub fn get_time(&self) -> String {
        format!("{:.3}s", self.time)
    }
}

impl FsNodes {
    pub fn reorder_tree(&mut self, mode: &ReOderMode) {
        match self {
            FsNodes::Dir(map) => {
                for (_, value) in map {
                    value.reorder_tree(mode);
                }
            }
            FsNodes::File { ast, .. } => match mode {
                ReOderMode::Expand => ast.expand(),
                ReOderMode::Flatten => ast.flatten(),
                ReOderMode::Sort => ast.sort_ast(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;

    use super::*;

    #[test]
    fn test_engine_lexer_vfs_fsdata_initialization() {
        let vfs = FsData::new("/dummy/path");

        assert_eq!(vfs.path, "/dummy/path");
        assert!(vfs.files.is_empty());
        if let FsNodes::Dir(map) = vfs.fsnodes {
            assert!(map.is_empty());
        } else {
            panic!("Der Root-Knoten muss ein Dir sein!");
        }
    }
    #[test]
    fn test_engine_lexer_vfs_get_files_single_nix_file() {
        let mut vfs = FsData::new("einzelne_datei.nix");
        vfs.get_files();

        assert_eq!(vfs.files.len(), 1);
        assert_eq!(vfs.files[0], "einzelne_datei.nix");
    }
    #[test]
    fn test_engine_lexer_vfs_gen_tree_with_real_files() {
        let temp_dir = env::temp_dir().join("prolyxena_test_vfs");
        let sub_dir = temp_dir.join("hosts").join("node1");
        fs::create_dir_all(&sub_dir).unwrap();
        let file_path = sub_dir.join("config.nix");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"true").unwrap();
        let mut vfs = FsData::new(temp_dir.to_str().unwrap());
        let _ = vfs.load();
        fs::remove_dir_all(&temp_dir).unwrap();
        assert_eq!(vfs.files.len(), 1);
        if let FsNodes::Dir(root_map) = &vfs.fsnodes {
            let hosts_node = root_map.get("hosts").expect("Ordner 'hosts' fehlt im Baum");

            if let FsNodes::Dir(hosts_map) = hosts_node {
                let node1_node = hosts_map.get("node1").expect("Ordner 'node1' fehlt im Baum");

                if let FsNodes::Dir(node1_map) = node1_node {
                    let config_file = node1_map.get("config.nix").expect("Datei 'config.nix' fehlt");
                    if let FsNodes::File { name, ast: _ } = config_file {
                        assert_eq!(name, "config.nix");
                    } else {
                        panic!("config.nix wurde nicht als FsNodes::File gespeichert!");
                    }
                } else {
                    panic!("node1 ist kein Ordner!");
                }
            } else {
                panic!("hosts ist kein Ordner!");
            }
        } else {
            panic!("Wurzel ist kein Ordner!");
        }
    }
}
