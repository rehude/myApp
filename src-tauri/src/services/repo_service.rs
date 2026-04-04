use crate::clients::GitHubClient;
use crate::error::AppError;
use crate::models::{Repo, RepoDetail};
use crate::state::AppState;

pub async fn get_repos(_state: &AppState, token: &str) -> Result<Vec<Repo>, AppError> {
    let client = GitHubClient::new();
    client.get_repos(token).await
}

pub async fn get_repo_detail(_state: &AppState, token: &str, owner: &str, repo: &str) -> Result<RepoDetail, AppError> {
    let client = GitHubClient::new();
    client.get_repo_detail(token, owner, repo).await
}
