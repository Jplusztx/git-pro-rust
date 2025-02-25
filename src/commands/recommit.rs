use crate::error::Result;
use crate::git::GitRepo;

pub fn execute(message: String) -> Result<()> {
    let repo = GitRepo::new()?;
    repo.recommit(&message)?;
    println!("Successfully updated the last commit message");
    Ok(())
}
