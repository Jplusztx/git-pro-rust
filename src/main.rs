use clap::{Parser, Subcommand};
mod commands;
mod error;
mod git;

#[derive(Parser)]
#[command(name = "git-pro")]
#[command(about = "A CLI tool to simplify git operations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Commit all changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
    /// Show commit logs
    Log {
        /// Number of logs to show
        #[arg(default_value_t = 10)]
        count: u32,
    },
    /// Modify the last commit message
    Recommit {
        /// New commit message
        #[arg(short, long)]
        message: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Commit { message } => commands::commit::execute(message),
        Commands::Log { count } => {
            println!("Showing {} logs", count);
            Ok(())
        }
        Commands::Recommit { message } => {
            println!("Modifying last commit with message: {}", message);
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
