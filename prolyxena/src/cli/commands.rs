use clap::{Parser as CalpParser, Subcommand};

use std::fs;

use crate::core::parsing::*;
use crate::cli::output::*;
use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::lexer::vfs::*;

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
    let mut data = FsData::new(&file);
    data.load();
    if output {
        println!("\n AST: \n{:#?}", data.fsnodes);
    }
    /*
    for (key, (ast, events)) in hashmap {
        if animation {
            println!("\n = = = Starte Parsing Animation für: {} = = =\n", key);
            show_parse_timeline(events);
        }
        
        if output {
            println!("\n AST für: {}: \n{:#?}", key, ast);
        }
    */
}
