use clap::{Parser as CalpParser, Subcommand};

use std::fs;

use crate::core::parsing::*;
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
    let hashmap = parse_rec(file);
    /*
    if animation {
        for (_key, value) in &hashmap {
            value.show_parse_timeline();
        }
    }
    */
    if output {
        for (key, value) in &hashmap {
            println!("Datei: {}: \n{:#?}", key, value);
        }
    }
}
