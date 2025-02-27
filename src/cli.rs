use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-pro")]
#[command(about = "A CLI tool to simplify git operations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    /// Branch management
    Branch {
        #[command(subcommand)]
        action: Option<BranchCommands>,
        /// Create and switch to a new branch
        #[arg(short = 's')]
        switch: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum BranchCommands {
    /// Create a new branch
    New {
        /// Branch name
        name: String,
        /// Base branch (default: current branch)
        #[arg(short = 'b', long)]
        base: Option<String>,
        /// Switch to the new branch after creation
        #[arg(short = 's', long)]
        switch: bool,
    },
    /// Delete a branch
    Del {
        /// Branch name
        name: String,
    },
    /// Rename a branch
    Rename {
        /// Old branch name
        old_name: String,
        /// New branch name
        new_name: String,
    },
    /// Delete branches matching regex pattern
    DelRegex {
        /// Regex pattern
        pattern: String,
        /// Force delete without confirmation
        #[arg(short, long)]
        force: bool,
    },
}
