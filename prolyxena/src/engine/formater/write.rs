use std::fs;
use std::path::{Path, PathBuf};

use crate::engine::formater::core::Format;
use crate::engine::lexer::vfs::*;

pub trait Write {
    fn walk_tree(&mut self) -> Result<(), String>;
    fn write_node(&self, node: &FsNodes, path: &Path, written_files: &mut Vec<PathBuf>) -> Result<(), String>;
}

impl Write for FsData {
    fn walk_tree(&mut self) -> Result<(), String> {
        let base_path = PathBuf::from(&self.path);
        if self.path.ends_with(".nix") {
            if let FsNodes::Dir(map) = &self.fsnodes {
                if let Some(FsNodes::File { ast, .. }) = map.values().next() {
                    let content = ast.format_nix(0);
                    fs::write(&base_path, content)
                        .map_err(|err| format!("Konnte Datei {:?} nicht beschreiben: {}", base_path, err))?;
                    return Ok(());
                }
            }
            return Err("Konnte die einzelne Datei nicht im AST finden".to_string());
        }

        let mut written_files: Vec<PathBuf> = Vec::new();
        self.write_node(&self.fsnodes, &base_path, &mut written_files)?;

        let canonical_written: Vec<PathBuf> = written_files
            .into_iter()
            .filter_map(|p| p.canonicalize().ok())
            .collect();

        if canonical_written.len() < self.files.len() {
            eprintln!(
                "NOTFALL-ABBRUCH: Es wurden nur {} Dateien generiert, aber {} alte Dateien gefunden! Der Cleaner wird blockiert, um Datenverlust zu verhindern.", 
                canonical_written.len(), 
                self.files.len()
            );
            return Err("Rebuild aus Sicherheitsgründen abgebrochen".to_string());
        }

        for old_file_str in &self.files {
            let old_path = PathBuf::from(old_file_str);
            if let Ok(canon) = old_path.canonicalize() {
                if !canonical_written.contains(&canon) {
                    if let Err(e) = fs::remove_file(&old_path) {
                        eprintln!("Warnung: Konnte veraltete Datei({:?}) nicht löschen: {}", old_path, e);
                    } else {
                        println!("Deleted => {:?}", old_path);
                    }
                }
            }
        }
        Ok(())
    }

    fn write_node(&self, node: &FsNodes, path: &Path, written_files: &mut Vec<PathBuf>) -> Result<(), String> {
        match node {
            FsNodes::Dir(map) => {
                if !path.exists() {
                    fs::create_dir_all(path)
                        .map_err(|err| format!("Konnte Ordner({:?}) nicht erstellen: {}", path, err))?;
                }

                for (name, child_node) in map {
                    let next_path = path.join(name);
                    self.write_node(child_node, &next_path, written_files)?;
                }
            }
            FsNodes::File { name: _, ast } => {
                let content = ast.format_nix(0);
                fs::write(path, content)
                    .map_err(|err| format!("Konnte Datei({:?}) nicht beschreiben: {}", path, err))?;
                written_files.push(path.to_path_buf());
            }
        }
        Ok(())
    }
}
