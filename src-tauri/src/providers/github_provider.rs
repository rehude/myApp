use crate::error::AppError;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail};
use crate::providers::GitProvider;
use crate::clients::http_client::build_client;
use async_trait::async_trait;

const GITHUB_API_BASE: &str = "https://api.github.com";

pub struct GitHubProvider {
    pub token: String,
}

impl GitHubProvider {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[async_trait]
impl GitProvider for GitHubProvider {
    async fn get_repos(&self) -> Result<Vec<Repo>, AppError> {
        let client = build_client();
        let url = format!("{}/user/repos?per_page=100", GITHUB_API_BASE);
        let response = client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown(format!("HTTP error status: {}", response.status())));
        }

        let repos: Vec<Repo> = response.json().await?;
        Ok(repos)
    }

    async fn get_repo_detail(&self, owner: String, repo: String) -> Result<RepoDetail, AppError> {
        let client = build_client();
        // If repo is empty, owner contains "owner/repo" format
        let (owner_str, repo_str) = if repo.is_empty() {
            let parts: Vec<&str> = owner.split('/').collect();
            if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                (owner.clone(), String::new())
            }
        } else {
            (owner, repo)
        };
        let url = format!("{}/repos/{}/{}", GITHUB_API_BASE, owner_str, repo_str);
        eprintln!("GitHub API URL: {}", url);

        let response = client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        eprintln!("GitHub response status: {}", response.status());

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown(format!("HTTP error status: {}", response.status())));
        }

        let repo_detail: RepoDetail = response.json().await?;
        Ok(repo_detail)
    }

    async fn get_commits(&self, owner: String, repo: String) -> Result<Vec<CommitSummary>, AppError> {
        let client = build_client();
        // If repo is empty, owner contains "owner/repo" format
        let (owner_str, repo_str) = if repo.is_empty() {
            let parts: Vec<&str> = owner.split('/').collect();
            if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                (owner.clone(), String::new())
            }
        } else {
            (owner, repo)
        };
        let url = format!("{}/repos/{}/{}/commits?per_page=100", GITHUB_API_BASE, owner_str, repo_str);
        eprintln!("GitHub API URL: {}", url);

        let response = client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        eprintln!("GitHub response status: {}", response.status());

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown(format!("HTTP error status: {}", response.status())));
        }

        let commits: Vec<CommitSummary> = response.json().await?;
        Ok(commits)
    }

    async fn get_commit_detail(&self, owner: String, repo: String, sha: String) -> Result<CommitDetail, AppError> {
        let client = build_client();
        // If repo is empty, owner contains "owner/repo" format
        let (owner_str, repo_str) = if repo.is_empty() {
            let parts: Vec<&str> = owner.split('/').collect();
            if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                (owner.clone(), String::new())
            }
        } else {
            (owner, repo)
        };
        let url = format!("{}/repos/{}/{}/commits/{}", GITHUB_API_BASE, owner_str, repo_str, sha);
        eprintln!("GitHub API URL: {}", url);

        let response = client
            .get(&url)
            .bearer_auth(&self.token)
            .header("User-Agent", "tauri-git-client")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        eprintln!("GitHub response status: {}", response.status());

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            return Err(AppError::Unknown(format!("HTTP error status: {}", response.status())));
        }

        let commit_detail: CommitDetail = response.json().await?;
        Ok(commit_detail)
    }
}
