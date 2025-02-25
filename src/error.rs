use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitProError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),

    #[error("Not a git repository")]
    NotAGitRepository,

    #[error("No commits found")]
    NoCommits,

    #[error("Failed to get repository: {0}")]
    RepositoryError(String),
}

pub type Result<T> = std::result::Result<T, GitProError>;
