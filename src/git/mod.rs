use crate::error::{GitProError, Result};
use git2::{Repository, RepositoryState, Status, StatusOptions, Time};
use std::fmt;

pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub time: Time,
}

impl fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.7} {} <{}> {}",
            self.id,
            self.time.seconds(),
            self.author,
            self.message.split('\n').next().unwrap_or("")
        )
    }
}

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn new() -> Result<Self> {
        Repository::open(".")
            .map(|repo| Self { repo })
            .map_err(|_| GitProError::NotAGitRepository)
    }

    pub fn check_repository_state(&self) -> Result<()> {
        match self.repo.state() {
            RepositoryState::Clean => Ok(()),
            _ => Err(GitProError::RepositoryError(
                "Repository is not in a clean state".to_string(),
            )),
        }
    }

    pub fn commit_all(&self, message: &str) -> Result<()> {
        // 检查仓库状态
        self.check_repository_state()?;

        // 获取仓库状态
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = self.repo.statuses(Some(&mut status_opts))?;

        if statuses.is_empty() {
            return Err(GitProError::NoCommits);
        }

        // 添加所有更改到暂存区
        let mut index = self.repo.index()?;
        for status in statuses.iter() {
            if status.status() != Status::CURRENT {
                // 使用 add_all 替代单个文件添加
            }
        }
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        // 创建提交
        let signature = self.repo.signature()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let parent = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None,
        };

        let parents: Vec<&git2::Commit> = match &parent {
            Some(commit) => vec![commit],
            None => vec![],
        };

        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(())
    }

    pub fn get_logs(&self, count: u32) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let commits: Vec<CommitInfo> = revwalk
            .take(count as usize)
            .filter_map(|id| id.ok())
            .filter_map(|id| self.repo.find_commit(id).ok())
            .map(|commit| CommitInfo {
                id: commit.id().to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                time: commit.time(),
            })
            .collect();

        if commits.is_empty() {
            return Err(GitProError::NoCommits);
        }

        Ok(commits)
    }

    pub fn recommit(&self, message: &str) -> Result<()> {
        // 获取最后一次提交
        let head = self.repo.head()?;
        let last_commit = head.peel_to_commit()?;

        // 获取原始提交信息
        let tree = last_commit.tree()?;

        // 创建新的提交
        let signature = self.repo.signature()?;
        last_commit.amend(
            Some("HEAD"),
            Some(&signature),
            Some(&signature),
            None,
            Some(message),
            Some(&tree),
        )?;

        Ok(())
    }

    pub fn list_branches(&self) -> Result<Vec<BranchInfo>> {
        let branches = self.repo.branches(None)?;
        let current = self.repo.head()?.shorthand().unwrap_or("").to_string();
        
        let branch_list = branches
            .map(|b| {
                let (branch, _) = b?;
                let name = branch.name()?.unwrap_or("").to_string();
                Ok(BranchInfo {
                    is_current: name == current,
                    name,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        
        Ok(branch_list)
    }

    pub fn create_branch(&self, name: &str) -> Result<()> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        self.repo.branch(name, &commit, false)?;
        Ok(())
    }

    pub fn delete_branch(&self, name: &str) -> Result<()> {
        let mut branch = self.repo.find_branch(name, git2::BranchType::Local)?;
        branch.delete()?;
        Ok(())
    }

    pub fn rename_branch(&self, old_name: &str, new_name: &str) -> Result<()> {
        let mut branch = self.repo.find_branch(old_name, git2::BranchType::Local)?;
        branch.rename(new_name, false)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
}

impl fmt::Display for BranchInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_current {
            write!(f, "* {}", self.name)
        } else {
            write!(f, "  {}", self.name)
        }
    }
}
