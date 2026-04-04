use crate::error::AppError;
use crate::models::{CommitDetail, CommitSummary, Repo, RepoDetail};

const GITHUB_API_BASE: &str = "https://api.github.com";

pub struct GitHubClient {
    client: reqwest::Client,
}

impl GitHubClient {
    pub fn new() -> Self {
        Self {
            client: crate::clients::http_client::build_client(),
        }
    }

    pub async fn get_repos(&self, token: &str) -> Result<Vec<Repo>, AppError> {
        let url = format!("{}/user/repos?per_page=100", GITHUB_API_BASE);
        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown);
        }

        let repos: Vec<Repo> = response.json().await?;
        Ok(repos)
    }

    pub async fn get_repo_detail(&self, token: &str, owner: &str, repo: &str) -> Result<RepoDetail, AppError> {
        let url = format!("{}/repos/{}/{}", GITHUB_API_BASE, owner, repo);
        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown);
        }

        let repo_detail: RepoDetail = response.json().await?;
        Ok(repo_detail)
    }

    pub async fn get_commits(&self, token: &str, owner: &str, repo: &str) -> Result<Vec<CommitSummary>, AppError> {
        let url = format!("{}/repos/{}/{}/commits?per_page=100", GITHUB_API_BASE, owner, repo);
        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown);
        }

        let commits: Vec<CommitSummary> = response.json().await?;
        Ok(commits)
    }

    pub async fn get_commit_detail(&self, token: &str, owner: &str, repo: &str, sha: &str) -> Result<CommitDetail, AppError> {
        let url = format!("{}/repos/{}/{}/commits/{}", GITHUB_API_BASE, owner, repo, sha);
        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown);
        }

        let commit_detail: CommitDetail = response.json().await?;
        Ok(commit_detail)
    }
}

impl Default for GitHubClient {
    fn default() -> Self {
        Self::new()
    }
}
