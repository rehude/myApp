use crate::error::AppError;
use crate::models::{CommitSummary, CommitDetail};
use crate::providers::{GitProvider, GitHubProvider, GitLabProvider};
use crate::state::AppState;
use crate::services::auth_service;

pub async fn get_commits(state: &AppState, owner: String, repo: String) -> Result<Vec<CommitSummary>, AppError> {
    let provider = auth_service::get_provider(state);
    let token = auth_service::get_token(state, provider).ok_or(AppError::NotLoggedIn)?;

    let provider_impl: Box<dyn GitProvider> = match provider {
        crate::state::Provider::GitHub => Box::new(GitHubProvider::new(token)),
        crate::state::Provider::GitLab => {
            let gitlab_url = auth_service::get_gitlab_url(state);
            Box::new(GitLabProvider::new(token, gitlab_url))
        }
    };

    // For GitLab, owner is the project ID; repo is empty
    // For GitHub, owner and repo are the standard parameters
    provider_impl.get_commits(owner, repo).await
}

pub async fn get_commit_detail(state: &AppState, owner: String, repo: String, sha: String) -> Result<CommitDetail, AppError> {
    let provider = auth_service::get_provider(state);
    let token = auth_service::get_token(state, provider).ok_or(AppError::NotLoggedIn)?;

    let provider_impl: Box<dyn GitProvider> = match provider {
        crate::state::Provider::GitHub => Box::new(GitHubProvider::new(token)),
        crate::state::Provider::GitLab => {
            let gitlab_url = auth_service::get_gitlab_url(state);
            Box::new(GitLabProvider::new(token, gitlab_url))
        }
    };

    // For GitLab, owner is the project ID; repo is empty
    // For GitHub, owner and repo are the standard parameters
    provider_impl.get_commit_detail(owner, repo, sha).await
}
