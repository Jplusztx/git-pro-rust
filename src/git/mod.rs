use crate::error::{GitProError, Result};
use git2::{Repository, RepositoryState, Status, StatusOptions};

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn new() -> Result<Self> {
        let repo = Repository::open(".")?;
        Ok(Self { repo })
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
            return Err(GitProError::RepositoryError(
                "No changes to commit".to_string(),
            ));
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
}
