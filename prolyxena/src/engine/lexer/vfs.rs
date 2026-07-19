use crate::engine::core::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FsData {
    files: Vec<String>,
    path: String,
    fsnodes: FsNodes,
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

    pub fn load(&mut self, path: &str) {
        self.get_files(path);
    }

    pub fn get_files(&mut self, path: &str) {
        self.files = WalkDir::new(path)
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
            .collect();
        }

    pub fn gen_tree(&mut self) {
        let files = self.files.clone();
        for i in files {
            let parts: Vec<&str> = i.split('/').collect();
            if parts.is_empty() {
                continue;
            }

            let mut pointer = &mut self.fsnodes;
            for j in 0..parts.len() - 1 {
                let folder = parts[j];
                if let FsNodes::Dir(ref mut map) = pointer {
                    pointer = map.entry(folder.to_string()).or_insert_with(|| {
                        FsNodes::Dir(HashMap::new())
                    });
                } else {
                    eprintln!("Fehler: Versucht einen Ordner in einer Datei zu erstellen");
                    break;
                }
            }
            let file_name = parts.last().unwrap();
            if let FsNodes::Dir(ref mut map) = pointer {
                let content = match fs::read_to_string(PathBuf::from(&self.path).join(&i)) {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Fehler beim Lesen der Datei: {}", e);
                        process::exit(1);
                    },
                };
                let mut file_data = Lexer::new(&content, i.clone());
                let ast = match file_data.parse_value() {
                    Ok(ast) => ast,
                    Err(e) => {
                        eprintln!("Fehler beim Lexen: \n{}", e);
                        process::exit(1);
                    },
                };
                map.insert(file_name.to_string(), FsNodes::File {
                    name: file_name.to_string(), 
                    ast,
                });
            }
        }
    }
}
