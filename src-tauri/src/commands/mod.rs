use tauri::State;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail};
use crate::services;
use crate::state::AppState;

#[tauri::command]
pub async fn set_token(state: State<'_, AppState>, token: String) -> Result<(), String> {
    let mut guard = state.github_token.lock().map_err(|e| e.to_string())?;
    *guard = Some(token);
    Ok(())
}

#[tauri::command]
pub async fn get_repos(state: State<'_, AppState>) -> Result<Vec<Repo>, String> {
    let token = {
        let guard = state.github_token.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("Not logged in")?
    };
    services::get_repos(&state, &token).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_repo_detail(state: State<'_, AppState>, owner: String, repo: String) -> Result<RepoDetail, String> {
    let token = {
        let guard = state.github_token.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("Not logged in")?
    };
    services::get_repo_detail(&state, &token, &owner, &repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_commits(state: State<'_, AppState>, owner: String, repo: String) -> Result<Vec<CommitSummary>, String> {
    let token = {
        let guard = state.github_token.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("Not logged in")?
    };
    services::get_commits(&state, &token, &owner, &repo).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_commit_detail(state: State<'_, AppState>, owner: String, repo: String, sha: String) -> Result<CommitDetail, String> {
    let token = {
        let guard = state.github_token.lock().map_err(|e| e.to_string())?;
        guard.clone().ok_or("Not logged in")?
    };
    services::get_commit_detail(&state, &token, &owner, &repo, &sha).await.map_err(|e| e.to_string())
}
