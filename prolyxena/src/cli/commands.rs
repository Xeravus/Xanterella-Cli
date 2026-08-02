use clap::{Parser as CalpParser, Subcommand};

use std::io::Write as StdOut;
use std::io::stdout;

use crate::engine::lexer::vfs::*;
use crate::engine::formater::write::Write;
use crate::tui::core::*;

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
        #[arg(short, long)]
        debug: bool,
    },
    Format {
        path: String,
        #[arg(short, long, conflicts_with = "output")]
        animation: bool,
        #[arg(short, long, conflicts_with = "animation")]
        output: bool,
        #[arg(short, long, conflicts_with = "animation")]
        time: bool,
        #[arg(short, long)]
        debug: bool,
    },
}

pub fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Show { path, animation, output, time, debug } => {
            let mut stdout = stdout();
            prolyxena_parse(&mut stdout, path.to_string(), *animation, *output, *time, *debug);
        }
        Commands::Format { path, animation, output, time, debug } => {
            let mut stdout = stdout();
            prolyxena_format(&mut stdout, path.to_string(), *animation, *output, *time, *debug);
        }
    }
}

pub fn prolyxena_parse(writer: &mut impl StdOut, file: String, animation: bool, output: bool, time: bool, debug: bool) {
    if animation {
        let mut tui = Tui::new(false, debug);
        tui.load(&file);
    } else {
        let mut data = FsData::new(&file);
        data.load();
        if output {
            let _ = writeln!(writer, "{:#?}", data.fsnodes);
        }
        if time {
            let _ = writeln!(writer, "Time: {}", data.get_time());
        }
    }
}

pub fn prolyxena_format(writer: &mut impl StdOut, file: String, animation: bool, output: bool, time: bool, debug: bool) {
    if animation {
        let mut tui = Tui::new(true, debug);
        tui.load(&file);
    } else {
        let mut data = FsData::new(&file);
        data.sort(true);
        data.load();
        let _ = data.walk_tree();
        if output {
            let _ = writeln!(writer, "{:#?}", data.fsnodes);
        }
        if time {
            let _ = writeln!(writer, "Time: {}", data.get_time());
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
