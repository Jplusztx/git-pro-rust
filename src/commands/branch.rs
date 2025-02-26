use crate::cli::BranchCommands;
use crate::error::Result;
use crate::git::GitRepo;

pub fn execute(action: Option<BranchCommands>) -> Result<()> {
    let repo = GitRepo::new()?;

    match action {
        None => list_branches(&repo),
        Some(cmd) => match cmd {
            BranchCommands::New { name, base } => create_branch(&repo, &name, base.as_deref()),
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

fn create_branch(repo: &GitRepo, name: &str, base: Option<&str>) -> Result<()> {
    repo.create_branch_from(name, base)?;
    match base {
        Some(base_branch) => println!("Created branch '{}' from '{}'", name, base_branch),
        None => println!("Created branch '{}'", name),
    }
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
