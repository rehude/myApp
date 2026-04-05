use tauri::State;
use crate::models::{Repo, RepoDetail, CommitSummary, CommitDetail};
use crate::services;
use crate::state::{AppState, Provider};
use crate::error::ApiResponse;
use crate::services::auth_service::{self, AccountInfo};

#[derive(serde::Serialize)]
pub struct InitAppResponse {
    pub logged_in: bool,
    pub provider: String,
    pub current_token_exists: bool,
    pub has_saved_accounts: bool,
    pub github_accounts: Vec<AccountInfo>,
    pub gitlab_accounts: Vec<AccountInfo>,
}

#[tauri::command]
pub async fn init_app(state: State<'_, AppState>) -> Result<InitAppResponse, String> {
    let current_state = auth_service::get_current_state(&state);

    Ok(InitAppResponse {
        logged_in: current_state.current_token_exists,
        provider: current_state.provider,
        current_token_exists: current_state.current_token_exists,
        has_saved_accounts: current_state.has_saved_accounts,
        github_accounts: current_state.github_accounts,
        gitlab_accounts: current_state.gitlab_accounts,
    })
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> Result<(), String> {
    auth_service::logout(&state);
    Ok(())
}

#[tauri::command]
pub async fn save_account(
    state: State<'_, AppState>,
    provider: String,
    token: String,
    gitlab_url: Option<String>,
    label: String,
) -> Result<ApiResponse<AccountInfo>, String> {
    let p = match provider.as_str() {
        "github" => Provider::GitHub,
        "gitlab" => Provider::GitLab,
        _ => return Ok(ApiResponse::error("Invalid provider".to_string(), "INVALID_PROVIDER")),
    };

    match auth_service::save_account(&state, p, token, gitlab_url, label) {
        Ok(account) => Ok(ApiResponse::success(AccountInfo {
            id: account.id,
            label: account.label,
            gitlab_url: account.gitlab_url,
        })),
        Err(e) => Ok(ApiResponse::error(e, "SAVE_ACCOUNT_ERROR")),
    }
}

#[tauri::command]
pub async fn get_accounts(provider: String) -> Result<Vec<AccountInfo>, String> {
    let p = match provider.as_str() {
        "github" => Provider::GitHub,
        "gitlab" => Provider::GitLab,
        _ => return Err("Invalid provider".to_string()),
    };

    let accounts = auth_service::get_accounts(p);
    Ok(accounts.into_iter().map(|a| AccountInfo {
        id: a.id,
        label: a.label,
        gitlab_url: a.gitlab_url,
    }).collect())
}

#[tauri::command]
pub async fn delete_account(account_id: String) -> Result<(), String> {
    auth_service::delete_account(&account_id)
}

#[tauri::command]
pub async fn set_current_account(
    state: State<'_, AppState>,
    account_id: String,
) -> Result<ApiResponse<AccountInfo>, String> {
    match auth_service::set_current_account(&state, &account_id) {
        Ok(account) => Ok(ApiResponse::success(AccountInfo {
            id: account.id,
            label: account.label,
            gitlab_url: account.gitlab_url,
        })),
        Err(e) => Ok(ApiResponse::error(e, "ACCOUNT_NOT_FOUND")),
    }
}

#[tauri::command]
pub async fn get_current_state(state: State<'_, AppState>) -> Result<InitAppResponse, String> {
    let current_state = auth_service::get_current_state(&state);

    Ok(InitAppResponse {
        logged_in: current_state.current_token_exists,
        provider: current_state.provider,
        current_token_exists: current_state.current_token_exists,
        has_saved_accounts: current_state.has_saved_accounts,
        github_accounts: current_state.github_accounts,
        gitlab_accounts: current_state.gitlab_accounts,
    })
}

#[tauri::command]
pub async fn set_provider(state: State<'_, AppState>, provider: String) -> Result<(), String> {
    let p = match provider.as_str() {
        "github" => Provider::GitHub,
        "gitlab" => Provider::GitLab,
        _ => return Err("Invalid provider".to_string()),
    };

    // Update provider
    {
        let mut guard = state.provider.lock().unwrap();
        *guard = p;
    }

    // Find an account of the new provider and switch to it
    let data = crate::persist::load_data();
    let target_account = data.accounts.iter().find(|a| a.provider == provider);

    if let Some(account) = target_account {
        // Switch token
        {
            let mut t = state.current_token.lock().unwrap();
            *t = Some(account.token.clone());
        }
        // Switch gitlab_url if applicable
        if let Some(ref url) = account.gitlab_url {
            let mut g = state.gitlab_url.lock().unwrap();
            *g = url.clone();
        }
        // Update last_used_id
        let mut data = crate::persist::load_data();
        data.last_used_id = Some(account.id.clone());
        crate::persist::save_data(&data).ok();
    } else {
        // No account for this provider, clear token
        let mut t = state.current_token.lock().unwrap();
        *t = None;
    }

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
