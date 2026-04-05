use crate::error::AppError;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail, CommitFile, CommitAuthor};
use crate::providers::GitProvider;
use crate::clients::http_client::build_client;
use async_trait::async_trait;
use serde::Deserialize;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

pub struct GitLabProvider {
    pub token: String,
    pub api_base: String,
}

impl GitLabProvider {
    pub fn new(token: String, api_base: String) -> Self {
        Self { token, api_base }
    }
}

#[derive(Debug, Deserialize)]
struct GitLabProject {
    id: u64,
    name: String,
    path_with_namespace: String,
    #[serde(rename = "visibility", default = "default_visibility")]
    visibility: String,
    web_url: String,
    description: Option<String>,
    star_count: i64,
    forks_count: i64,
    #[serde(default)]
    owner: Option<GitLabOwner>,
}

fn default_visibility() -> String {
    "private".to_string()
}

fn is_private_from_visibility(visibility: &str) -> bool {
    visibility == "private"
}

#[derive(Debug, Deserialize)]
struct GitLabOwner {
    #[serde(default)]
    _username: String,
    #[serde(default)]
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitLabCommit {
    id: String,
    message: String,
    author_name: String,
    author_email: String,
    authored_date: String,
    web_url: String,
}

#[derive(Debug, Deserialize)]
struct GitLabDiff {
    diff: String,
    new_path: String,
    #[serde(default)]
    _old_path: String,
}

fn map_gitlab_project_to_repo(project: GitLabProject) -> Repo {
    Repo {
        id: project.id as i64,
        name: project.name,
        full_name: project.path_with_namespace.clone(),
        is_private: is_private_from_visibility(&project.visibility),
        html_url: project.web_url,
        description: project.description,
        owner: crate::models::Owner {
            login: project.path_with_namespace.split('/').next().unwrap_or("").to_string(),
            avatar_url: project.owner.and_then(|o| o.avatar_url),
        },
    }
}

fn map_gitlab_project_to_detail(project: GitLabProject) -> RepoDetail {
    RepoDetail {
        id: project.id as i64,
        name: project.name,
        full_name: project.path_with_namespace.clone(),
        is_private: is_private_from_visibility(&project.visibility),
        html_url: project.web_url,
        description: project.description,
        stargazers_count: project.star_count,
        forks_count: project.forks_count,
        language: None,
        owner: crate::models::Owner {
            login: project.path_with_namespace.split('/').next().unwrap_or("").to_string(),
            avatar_url: project.owner.and_then(|o| o.avatar_url),
        },
    }
}

#[async_trait]
impl GitProvider for GitLabProvider {
    async fn get_repos(&self) -> Result<Vec<Repo>, AppError> {
        let client = build_client();
        let url = format!("{}/api/v4/projects?membership=true&per_page=100", self.api_base);
        
        let response = client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.token)
            .header("User-Agent", "tauri-git-client")
            .send()
            .await?;

        
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            let status = response.status();
            return Err(AppError::Unknown(format!("HTTP {}", status)));
        }

        let projects: Vec<GitLabProject> = response.json().await?;
        let repos = projects.into_iter().map(map_gitlab_project_to_repo).collect();
        Ok(repos)
    }

    async fn get_repo_detail(&self, owner: String, repo: String) -> Result<RepoDetail, AppError> {
        let client = build_client();
        // For GitLab, use owner as project_id (when repo is empty) or repo as project_id
        let project_id = if repo.is_empty() {
            // owner contains the project ID
            owner
        } else if repo.chars().all(|c| c.is_ascii_digit()) {
            repo
        } else {
            // URL encode the project identifier
            percent_encode(repo.as_bytes(), NON_ALPHANUMERIC).to_string()
        };
        let url = format!("{}/api/v4/projects/{}", self.api_base, project_id);
                
        let response = client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.token)
            .header("User-Agent", "tauri-git-client")
            .send()
            .await?;

        
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            let status = response.status();
            return Err(AppError::Unknown(format!("HTTP {}", status)));
        }

        let project: GitLabProject = response.json().await?;
        Ok(map_gitlab_project_to_detail(project))
    }

    async fn get_commits(&self, owner: String, repo: String) -> Result<Vec<CommitSummary>, AppError> {
        let client = build_client();
        // For GitLab, use owner as project_id (when repo is empty) or repo as project_id
        let project_id = if repo.is_empty() {
            owner
        } else if repo.chars().all(|c| c.is_ascii_digit()) {
            repo
        } else {
            percent_encode(repo.as_bytes(), NON_ALPHANUMERIC).to_string()
        };
        let url = format!("{}/api/v4/projects/{}/repository/commits?per_page=100", self.api_base, project_id);
        
        let response = client
            .get(&url)
            .header("PRIVATE-TOKEN", &self.token)
            .header("User-Agent", "tauri-git-client")
            .send()
            .await?;

        
        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !response.status().is_success() {
            let status = response.status();
            return Err(AppError::Unknown(format!("HTTP {}", status)));
        }

        let commits: Vec<GitLabCommit> = response.json().await?;
        let summaries: Vec<CommitSummary> = commits.into_iter().map(|c| {
            CommitSummary {
                sha: c.id,
                commit: crate::models::CommitInfo {
                    message: c.message,
                    author: CommitAuthor {
                        name: Some(c.author_name),
                        email: Some(c.author_email),
                        date: Some(c.authored_date),
                    },
                },
                html_url: c.web_url,
            }
        }).collect();
        Ok(summaries)
    }

    async fn get_commit_detail(&self, owner: String, repo: String, sha: String) -> Result<CommitDetail, AppError> {
        let client = build_client();
        // For GitLab, use owner as project_id (when repo is empty) or repo as project_id
        let project_id = if repo.is_empty() {
            owner
        } else if repo.chars().all(|c| c.is_ascii_digit()) {
            repo
        } else {
            percent_encode(repo.as_bytes(), NON_ALPHANUMERIC).to_string()
        };

        // First get commit info
        let commit_url = format!("{}/api/v4/projects/{}/repository/commits/{}", self.api_base, project_id, sha);

        let commit_response = client
            .get(&commit_url)
            .header("PRIVATE-TOKEN", &self.token)
            .header("User-Agent", "tauri-git-client")
            .send()
            .await?;

        
        if commit_response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized);
        }

        if !commit_response.status().is_success() {
            let status = commit_response.status();
            return Err(AppError::Unknown(format!("HTTP {}", status)));
        }

        let commit: GitLabCommit = serde_json::from_str(&commit_response.text().await?).map_err(|e| AppError::Unknown(e.to_string()))?;

        // Then get diffs using the separate diff endpoint
        let diff_url = format!("{}/api/v4/projects/{}/repository/commits/{}/diff", self.api_base, project_id, sha);
        
        let diff_response = client
            .get(&diff_url)
            .header("PRIVATE-TOKEN", &self.token)
            .header("User-Agent", "tauri-git-client")
            .send()
            .await?;

        
        let files: Option<Vec<CommitFile>> = if diff_response.status().is_success() {
            let diffs: Vec<GitLabDiff> = diff_response.json().await.unwrap_or_default();
                        if diffs.is_empty() {
                None
            } else {
                Some(diffs.into_iter().map(|d| {
                    CommitFile {
                        filename: Some(d.new_path),
                        status: None,
                        patch: Some(d.diff),
                        additions: None,
                        deletions: None,
                    }
                }).collect())
            }
        } else {
            None
        };

        Ok(CommitDetail {
            sha: commit.id,
            commit: crate::models::CommitInfo {
                message: commit.message,
                author: CommitAuthor {
                    name: Some(commit.author_name),
                    email: Some(commit.author_email),
                    date: Some(commit.authored_date),
                },
            },
            html_url: commit.web_url,
            files,
        })
    }
}
