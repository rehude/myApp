use crate::clients::GitHubClient;
use crate::error::AppError;
use crate::models::{CommitDetail, CommitSummary};
use crate::state::AppState;

pub async fn get_commits(_state: &AppState, token: &str, owner: &str, repo: &str) -> Result<Vec<CommitSummary>, AppError> {
    let client = GitHubClient::new();
    client.get_commits(token, owner, repo).await
}

pub async fn get_commit_detail(_state: &AppState, token: &str, owner: &str, repo: &str, sha: &str) -> Result<CommitDetail, AppError> {
    let client = GitHubClient::new();
    client.get_commit_detail(token, owner, repo, sha).await
}
