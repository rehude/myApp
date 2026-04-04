use tauri::State;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail};
use crate::services;
use crate::state::{AppState, Provider};
use crate::services::auth_service;

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
pub async fn get_repos(state: State<'_, AppState>) -> Result<Vec<Repo>, String> {
    services::get_repos(&state).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_repo_detail(state: State<'_, AppState>, owner: String, repo: String) -> Result<RepoDetail, String> {
    services::get_repo_detail(&state, owner, repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_commits(state: State<'_, AppState>, owner: String, repo: String) -> Result<Vec<CommitSummary>, String> {
    services::get_commits(&state, owner, repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_commit_detail(state: State<'_, AppState>, owner: String, repo: String, sha: String) -> Result<CommitDetail, String> {
    services::get_commit_detail(&state, owner, repo, sha).await.map_err(|e| e.to_string())
}
