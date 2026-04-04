use crate::error::AppError;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail};
use async_trait::async_trait;

#[async_trait]
pub trait GitProvider: Send + Sync {
    async fn get_repos(&self) -> Result<Vec<Repo>, AppError>;
    async fn get_repo_detail(&self, owner: String, repo: String) -> Result<RepoDetail, AppError>;
    async fn get_commits(&self, owner: String, repo: String) -> Result<Vec<CommitSummary>, AppError>;
    async fn get_commit_detail(&self, owner: String, repo: String, sha: String) -> Result<CommitDetail, AppError>;
}
