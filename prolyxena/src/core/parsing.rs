use walkdir::WalkDir;

use std::collections::HashMap;
use std::fs;
use std::process;

use crate::engine::lexer::core::*;
use crate::engine::core::*;

pub fn parse(path: String) -> NixValue {
    let content = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Fehler beim Lesen der Datei: {}", e);
            process::exit(1);
        },
    };
    let mut prolyxena = Lexer::new(&content, path);
    match prolyxena.parse_value() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Fehler beim Lexen: \n{}", e);
            process::exit(1);
        },
    }
}

pub fn parse_rec(folder: String) -> HashMap<String, NixValue> {
    let files: Vec<String> = WalkDir::new(folder)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            entry.path().to_str().map(|s| s.to_string())
        })
        .collect();
    let mut output: HashMap<String, NixValue> = HashMap::new();
    for i in files {
        output.insert(i.clone(), parse(i));
    }
    output
}
