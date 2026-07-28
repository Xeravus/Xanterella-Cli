use clap::{Parser as CalpParser, Subcommand};

use std::fs;

use crate::engine::core::*;
use crate::engine::lexer::core::*;
use crate::engine::lexer::vfs::*;

use crate::tui::core::*;

use std::{
    sync::mpsc, thread, time::Duration
};

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
        #[arg(short, long, conflicts_with = "output")]
        animation: bool,
        #[arg(short, long, conflicts_with = "animation")]
        output: bool,
        #[arg(short, long, conflicts_with = "animation")]
        time: bool,
    },
}

pub fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Show { path, animation, output, time } => {
            prolyxena_parse(path.to_string(), *animation, *output, *time);
        },
    }
}

pub fn prolyxena_parse(file: String, animation: bool, output: bool, time: bool) {
    if animation {
        let mut tui = Tui::new();
        tui.load(&file);
    } else {
        let mut data = FsData::new(&file);
        data.load();
        if output {
            println!("\n AST: \n{:#?}", data.fsnodes);
        }
        if time {
            println!("Time: {}", data.get_time());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
