use crate::persist::Account;
use crate::state::{AppState, Provider};

pub fn get_provider(state: &AppState) -> Provider {
    let guard = state.provider.lock().unwrap();
    *guard
}

pub fn get_token(state: &AppState, _provider: Provider) -> Option<String> {
    let guard = state.current_token.lock().unwrap();
    guard.clone()
}

pub fn get_gitlab_url(state: &AppState) -> String {
    let guard = state.gitlab_url.lock().unwrap();
    guard.clone()
}

pub fn save_account(
    state: &AppState,
    provider: Provider,
    token: String,
    gitlab_url: Option<String>,
    label: String,
) -> Result<Account, String> {
    let mut data = crate::persist::load_data();

    // Generate account id
    let id = format!("{}_{}", provider.as_str(), uuid_simple());

    let account = Account {
        id: id.clone(),
        provider: provider.as_str().to_string(),
        token,
        gitlab_url,
        label,
    };

    data.accounts.push(account.clone());
    data.last_used_id = Some(id.clone());

    crate::persist::save_data(&data)?;

    // Update current state
    {
        let mut p = state.provider.lock().unwrap();
        *p = provider;
    }
    {
        let mut t = state.current_token.lock().unwrap();
        *t = Some(account.token.clone());
    }

    data.accounts.into_iter().find(|a| a.id == id).ok_or("Account not found".to_string())
}

pub fn get_accounts(provider: Provider) -> Vec<Account> {
    let data = crate::persist::load_data();
    data.accounts
        .into_iter()
        .filter(|a| a.provider == provider.as_str())
        .collect()
}

pub fn delete_account(account_id: &str) -> Result<(), String> {
    let mut data = crate::persist::load_data();
    data.accounts.retain(|a| a.id != account_id);

    // If we deleted the last used, clear it
    if data.last_used_id.as_deref() == Some(account_id) {
        data.last_used_id = data.accounts.first().map(|a| a.id.clone());
    }

    crate::persist::save_data(&data)
}

pub fn set_current_account(state: &AppState, account_id: &str) -> Result<Account, String> {
    let data = crate::persist::load_data();

    let account = data.accounts.iter().find(|a| a.id == account_id)
        .ok_or("Account not found")?
        .clone();

    // Update state
    {
        let mut p = state.provider.lock().unwrap();
        *p = Provider::from_str(&account.provider);
    }
    {
        let mut t = state.current_token.lock().unwrap();
        *t = Some(account.token.clone());
    }
    if let Some(ref url) = account.gitlab_url {
        let mut g = state.gitlab_url.lock().unwrap();
        *g = url.clone();
    }

    // Update last used
    let mut data = crate::persist::load_data();
    data.last_used_id = Some(account_id.to_string());
    crate::persist::save_data(&data)?;

    Ok(account)
}

#[derive(serde::Serialize)]
pub struct CurrentState {
    pub provider: String,
    pub current_token_exists: bool,
    pub has_saved_accounts: bool,
    pub github_accounts: Vec<AccountInfo>,
    pub gitlab_accounts: Vec<AccountInfo>,
}

#[derive(serde::Serialize)]
pub struct AccountInfo {
    pub id: String,
    pub label: String,
    pub gitlab_url: Option<String>,
}

pub fn get_current_state(state: &AppState) -> CurrentState {
    let data = crate::persist::load_data();

    let provider = {
        let p = state.provider.lock().unwrap();
        p.as_str().to_string()
    };

    let current_token_exists = {
        let t = state.current_token.lock().unwrap();
        t.is_some()
    };

    let github_accounts: Vec<AccountInfo> = data.accounts
        .iter()
        .filter(|a| a.provider == "github")
        .map(|a| AccountInfo {
            id: a.id.clone(),
            label: a.label.clone(),
            gitlab_url: None,
        })
        .collect();

    let gitlab_accounts: Vec<AccountInfo> = data.accounts
        .iter()
        .filter(|a| a.provider == "gitlab")
        .map(|a| AccountInfo {
            id: a.id.clone(),
            label: a.label.clone(),
            gitlab_url: a.gitlab_url.clone(),
        })
        .collect();

    CurrentState {
        provider,
        current_token_exists,
        has_saved_accounts: !data.accounts.is_empty(),
        github_accounts,
        gitlab_accounts,
    }
}

pub fn logout(state: &AppState) {
    let mut t = state.current_token.lock().unwrap();
    *t = None;
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now)
}
