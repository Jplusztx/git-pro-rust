use crate::cli::BranchCommands;
use crate::error::Result;
use crate::git::GitRepo;

pub fn execute(action: Option<BranchCommands>, switch: Option<String>) -> Result<()> {
    let repo = GitRepo::new()?;

    // 处理快速切换
    if let Some(branch_name) = switch {
        if repo.branch_exists(&branch_name) {
            repo.switch_branch(&branch_name)?;
            println!("Switched to branch '{}'", branch_name);
        } else {
            create_branch(&repo, &branch_name, None, true)?;
        }
        return Ok(());
    }

    match action {
        Some(cmd) => match cmd {
            BranchCommands::New { name, base, switch } => {
                create_branch(&repo, &name, base.as_deref(), switch)
            }
            BranchCommands::Del { name } => delete_branch(&repo, &name),
            BranchCommands::Rename { old_name, new_name } => {
                rename_branch(&repo, &old_name, &new_name)
            }
            BranchCommands::DelRegex { pattern, force } => {
                delete_branches_by_regex(&repo, &pattern, force)
            }
        },
        None => list_branches(&repo),
    }
}

fn list_branches(repo: &GitRepo) -> Result<()> {
    let branches = repo.list_branches()?;
    for branch in branches {
        println!("{}", branch);
    }
    Ok(())
}

fn create_branch(repo: &GitRepo, name: &str, base: Option<&str>, switch: bool) -> Result<()> {
    repo.create_branch_from(name, base)?;
    match base {
        Some(base_branch) => println!("Created branch '{}' from '{}'", name, base_branch),
        None => println!("Created branch '{}'", name),
    }

    if switch {
        repo.switch_branch(name)?;
        println!("Switched to branch '{}'", name);
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

fn delete_branches_by_regex(repo: &GitRepo, pattern: &str, force: bool) -> Result<()> {
    let deleted = repo.delete_branches_by_pattern(pattern, force)?;

    if deleted.is_empty() {
        println!("No branches matched pattern '{}'", pattern);
    } else {
        println!("Deleted {} branches:", deleted.len());
        for branch in deleted {
            println!("  {}", branch);
        }
    }

    Ok(())
}
