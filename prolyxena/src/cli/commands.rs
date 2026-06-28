use clap::{Parser as CalpParser, Subcommand};

use std::fs;

use crate::engine::core::*;
use crate::engine::lexer::core::*;

#[derive(CalpParser)]
#[command(name = "Prolyxena")]
#[command(about = "Nix & NixOS Configuration Engine to parse & generate Nix Configurations", long_about = None)]


pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Show {
        #[arg(short, long)]
        path: String,
    },
}

pub fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Show { path } => {
            prolyxena_parse(path.to_string());
        },
    }
}

pub fn prolyxena_parse(file: String) {
    println!("Lese Datei: {}", file);
    let content = match fs::read_to_string(&file) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Fehler beim Lesen der Datei: {}", e);
            return;
        }
    };
    let mut prolyxena = Parser::new(&content);
    match prolyxena.parse_value() {
        Ok(ast) => {
            println!("Erfolgreich geparst");
            println!("{:#?}", ast);
        },
        Err(e) => {
            eprintln!("Parse-Fehler: {}", e);
        }
    }
}
