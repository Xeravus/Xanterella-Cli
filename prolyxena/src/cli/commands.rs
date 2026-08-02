use std::io::Write as StdOut;
use std::io::stdout;

use clap::{Parser as CalpParser, Subcommand};

use crate::engine::formater::write::Write;
use crate::engine::lexer::vfs::*;
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

        #[arg(short, long, conflicts_with = "flatten")]
        expand: bool,
        #[arg(short, long, conflicts_with = "expand")]
        flatten: bool,
        #[arg(short, long)]
        sort: bool,

        #[arg(short, long, conflicts_with = "output")]
        animation: bool,
        #[arg(short, long, conflicts_with = "animation")]
        output: bool,
        #[arg(short, long, conflicts_with = "animation")]
        time: bool,
        #[arg(short, long, requires = "animation")]
        debug: bool,
    },
    Format {
        path: String,

        #[arg(short, long, conflicts_with = "flatten")]
        expand: bool,
        #[arg(short, long, conflicts_with = "expand")]
        flatten: bool,
        #[arg(short, long)]
        sort: bool,

        #[arg(short, long, conflicts_with = "output")]
        animation: bool,
        #[arg(short, long, conflicts_with = "animation")]
        output: bool,
        #[arg(short, long, conflicts_with = "animation")]
        time: bool,
        #[arg(short, long, requires = "animation")]
        debug: bool,
    },
}

pub fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Show { path, expand, flatten, sort, animation, output, time, debug } => {
            let mut stdout = stdout();
            prolyxena_parse(
                &mut stdout,
                path.to_string(),
                *expand,
                *flatten,
                *sort,
                *animation,
                *output,
                *time,
                *debug,
            );
        }
        Commands::Format { path, expand, flatten, sort, animation, output, time, debug } => {
            let mut stdout = stdout();
            prolyxena_format(
                &mut stdout,
                path.to_string(),
                *expand,
                *flatten,
                *sort,
                *animation,
                *output,
                *time,
                *debug,
            );
        }
    }
}

pub fn prolyxena_parse(
    writer: &mut impl StdOut, file: String, expand: bool, flatten: bool, sort: bool, animation: bool, output: bool,
    time: bool, debug: bool,
) {
    if animation {
        let mut tui = Tui::new();
        tui.set_debug(debug);
        tui.set_sort(sort);
        tui.set_expand(expand);
        tui.set_flatten(flatten);
        tui.load(&file);
    } else {
        let mut data = FsData::new(&file);
        data.set_sort(sort);
        data.set_expand(expand);
        data.set_flatten(flatten);
        data.load();
        if output {
            let _ = writeln!(writer, "{:#?}", data.fsnodes);
        }
        if time {
            let _ = writeln!(writer, "Time: {}", data.get_time());
        }
    }
}

pub fn prolyxena_format(
    writer: &mut impl StdOut, file: String, expand: bool, flatten: bool, sort: bool, animation: bool, output: bool,
    time: bool, debug: bool,
) {
    if animation {
        let mut tui = Tui::new();
        tui.set_debug(debug);
        tui.set_sort(sort);
        tui.set_expand(expand);
        tui.set_flatten(flatten);
        tui.load(&file);
    } else {
        let mut data = FsData::new(&file);
        data.set_sort(sort);
        data.set_expand(expand);
        data.set_flatten(flatten);
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
