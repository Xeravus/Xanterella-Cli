use crate::engine::core::*;
use crate::engine::lexer::core::*;

use walkdir::WalkDir;

use std::collections::HashMap;
use std::fs;
use std::process;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FsData {
    pub files: Vec<String>,
    pub path: String,
    pub fsnodes: FsNodes,
}

#[derive(Debug, Clone)]
pub enum FsNodes {
    File {
        name: String,
        ast: NixValue,
    },
    Dir(HashMap<String, FsNodes>),
}

impl FsData {
    pub fn new(path: &str) -> Self {
        FsData {
            files: vec![],
            path: path.to_string(),
            fsnodes: FsNodes::Dir(HashMap::new()),
        }
    }

    pub fn load(&mut self) {
        self.get_files();
        self.gen_tree();
    }

    pub fn get_files(&mut self) {
        let files: Vec<String> = if !self.path.ends_with(".nix") {
            WalkDir::new(&self.path)
                .min_depth(1)
                .sort_by_file_name()
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter_map(|entry| {
                    let path_str = entry.path().to_str()?;
                    if path_str.ends_with(".nix") {
                        Some(path_str.to_string())
                    } else {
                        None
                    }
                })
                .collect()
            } else if self.path.ends_with(".nix") {
                vec![
                    self.path.to_string()
                ]
            } else {
                eprintln!("Fehler: Keine Nix Datei(n) angegeben");
                process::exit(1);
                vec![]
            };
        self.files = files;
    }

    pub fn gen_tree(&mut self) {
        let files = self.files.clone();
        for i in files {
            let rel_path = i.strip_prefix(&self.path).unwrap_or(&i);
            let clean_path = rel_path.trim_start_matches('/');
            let parts: Vec<&str> = clean_path.split('/').collect();
            if parts.is_empty() {
                continue;
            }

            let mut pointer = &mut self.fsnodes;
            for j in 0..parts.len() - 1 {
                let folder = parts[j];
                if let FsNodes::Dir(map) = pointer {
                    pointer = map.entry(folder.to_string()).or_insert_with(|| {
                        FsNodes::Dir(HashMap::new())
                    });
                } else {
                    eprintln!("Fehler: Versucht einen Ordner in einer Datei zu erstellen");
                    break;
                }
            }
            let file_name = parts.last().unwrap();
            if let FsNodes::Dir(map) = pointer {
                let content = fs::read_to_string(&i).unwrap();
                let mut file_data = Lexer::new(&content, i.clone());
                let ast = file_data.parse_value().unwrap();
                map.insert(file_name.to_string(), FsNodes::File {
                    name: file_name.to_string(), 
                    ast
                });
            }
        }
    }
}

#[cfg(test)]
#[path = "vfs_test.rs"]
mod tests;
