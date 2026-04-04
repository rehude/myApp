use tauri::State;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail};
use crate::services;
use crate::state::{AppState, Provider};
use crate::error::ApiResponse;
use crate::services::auth_service;

#[derive(serde::Serialize)]
pub struct InitAppResponse {
    pub logged_in: bool,
    pub provider: String,
    pub github_token_exists: bool,
    pub gitlab_token_exists: bool,
}

#[tauri::command]
pub async fn init_app(state: State<'_, AppState>) -> Result<InitAppResponse, String> {
    let provider = auth_service::get_provider(&state);
    let github_token = auth_service::get_token(&state, Provider::GitHub);
    let gitlab_token = auth_service::get_token(&state, Provider::GitLab);

    let provider_str = match provider {
        Provider::GitHub => "github",
        Provider::GitLab => "gitlab",
    };

    let logged_in = github_token.is_some() || gitlab_token.is_some();

    Ok(InitAppResponse {
        logged_in,
        provider: provider_str.to_string(),
        github_token_exists: github_token.is_some(),
        gitlab_token_exists: gitlab_token.is_some(),
    })
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    auth_service::clear_tokens(&state);
    Ok(())
}

#[tauri::command]
pub async fn set_provider(state: State<'_, AppState>, provider: String) -> Result<(), String> {
    let p = match provider.as_str() {
        "github" => Provider::GitHub,
        "gitlab" => Provider::GitLab,
        _ => return Err("Invalid provider".to_string()),
    };
    auth_service::set_provider(&state, p);
    Ok(())
}

#[tauri::command]
pub async fn set_github_token(state: State<'_, AppState>, token: String) -> Result<(), String> {
    auth_service::set_token(&state, token, Provider::GitHub);
    Ok(())
}

#[tauri::command]
pub async fn set_gitlab_token(state: State<'_, AppState>, token: String) -> Result<(), String> {
    auth_service::set_token(&state, token, Provider::GitLab);
    Ok(())
}

#[tauri::command]
pub async fn set_gitlab_url(state: State<'_, AppState>, url: String) -> Result<(), String> {
    auth_service::set_gitlab_url(&state, url);
    Ok(())
}

#[tauri::command]
pub async fn get_repos(state: State<'_, AppState>) -> Result<ApiResponse<Vec<Repo>>, String> {
    match services::get_repos(&state).await {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Ok(ApiResponse::from(e)),
    }
}

#[tauri::command]
pub async fn get_repo_detail(state: State<'_, AppState>, owner: String, repo: String) -> Result<ApiResponse<RepoDetail>, String> {
    match services::get_repo_detail(&state, owner, repo).await {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Ok(ApiResponse::from(e)),
    }
}

#[tauri::command]
pub async fn get_commits(state: State<'_, AppState>, owner: String, repo: String) -> Result<ApiResponse<Vec<CommitSummary>>, String> {
    match services::get_commits(&state, owner, repo).await {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Ok(ApiResponse::from(e)),
    }
}

#[tauri::command]
pub async fn get_commit_detail(state: State<'_, AppState>, owner: String, repo: String, sha: String) -> Result<ApiResponse<CommitDetail>, String> {
    match services::get_commit_detail(&state, owner, repo, sha).await {
        Ok(data) => Ok(ApiResponse::success(data)),
        Err(e) => Ok(ApiResponse::from(e)),
    }
}
