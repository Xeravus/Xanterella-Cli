use clap::{Parser as CalpParser, Subcommand};

use std::fs;

use crate::cli::output::*;
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
        #[arg(short, long)]
        animation: bool,
        #[arg(short, long)]
        output: bool,
    },
}

pub fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Show { path, animation, output } => {
            prolyxena_parse(path.to_string(), *animation, *output);
        },
    }
}

pub fn prolyxena_parse(file: String, animation: bool, output: bool) {
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
            if output {
                println!("{:#?}", ast);
            }
            if animation {
                prolyxena.show_parse_timeline();
            }
        },
        Err(e) => {
            eprintln!("Parse-Fehler: {}", e);
        }
    }
}
