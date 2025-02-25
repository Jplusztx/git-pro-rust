use crate::error::Result;
use crate::git::GitRepo;

pub fn execute(count: u32) -> Result<()> {
    let repo = GitRepo::new()?;
    let commits = repo.get_logs(count)?;

    for commit in commits {
        println!("{}", commit);
    }

    Ok(())
}
