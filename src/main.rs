mod cli;
mod commands;
mod error;
mod git;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Commit { message } => commands::commit::execute(message),
        Commands::Log { count } => commands::log::execute(count),
        Commands::Recommit { message } => commands::recommit::execute(message),
        Commands::Branch { action, switch } => commands::branch::execute(action, switch),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
