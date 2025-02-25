use crate::error::Result;
use crate::git::GitRepo;

pub fn execute(message: String) -> Result<()> {
    let repo = GitRepo::new()?;
    repo.commit_all(&message)?;
    println!("Successfully committed changes with message: {}", message);
    Ok(())
}
