use crate::engine::lexer::vfs::*;
use crate::engine::core::*;
use crate::engine::formater::core::Format;

use std::fs;

pub trait Write {
    fn walk_tree(&mut self) -> Result<(), String>;
}

impl Write for FsData {
    fn walk_tree(&mut self) -> Result<(), String> {
        for i in self.files.clone() {
            let rel_path = i.strip_prefix(&self.path).unwrap_or(&i);
            let clean_path = rel_path.trim_start_matches('/');
            let parts: Vec<&str> = clean_path.split('/').collect();
            let mut pointer = &mut self.fsnodes;
            for j in 0..parts.len() - 1 {
                let folder = parts[j];
                if let FsNodes::Dir(map) = pointer {
                    pointer = map. get_mut(folder).ok_or(format!("Cant Extract Values out of IndexMap"))?;
                } else {
                    return Err(format!("Tree is Corrupt"));
                }
            }
            let file_name = parts.last().unwrap().to_string();
            if let FsNodes::Dir(map) = pointer {
                let file = map.get(&file_name).unwrap();
                

                if let FsNodes::File { name: _, ast } = file {
                    write(ast, &i);
                }
            }
        }
        Ok(())
    }
}

pub fn write(ast: &NixValue, path: &String) {
    println!("{}", ast.format_nix(0));
    let _ = fs::write(path, ast.format_nix(0));
}
