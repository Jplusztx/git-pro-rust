use crate::error::Result;
use crate::git::GitRepo;
use crate::cli::BranchCommands;

pub fn execute(action: Option<BranchCommands>) -> Result<()> {
    let repo = GitRepo::new()?;

    match action {
        None => list_branches(&repo),
        Some(cmd) => match cmd {
            BranchCommands::New { name } => create_branch(&repo, &name),
            BranchCommands::Del { name } => delete_branch(&repo, &name),
            BranchCommands::Rename { old_name, new_name } => {
                rename_branch(&repo, &old_name, &new_name)
            }
        },
    }
}

fn list_branches(repo: &GitRepo) -> Result<()> {
    let branches = repo.list_branches()?;
    for branch in branches {
        println!("{}", branch);
    }
    Ok(())
}

fn create_branch(repo: &GitRepo, name: &str) -> Result<()> {
    repo.create_branch(name)?;
    println!("Created branch '{}'", name);
    Ok(())
}

fn delete_branch(repo: &GitRepo, name: &str) -> Result<()> {
    repo.delete_branch(name)?;
    println!("Deleted branch '{}'", name);
    Ok(())
}

fn rename_branch(repo: &GitRepo, old_name: &str, new_name: &str) -> Result<()> {
    repo.rename_branch(old_name, new_name)?;
    println!("Renamed branch '{}' to '{}'", old_name, new_name);
    Ok(())
}
