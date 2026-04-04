use crate::state::{AppState, Provider};

pub fn set_token(state: &AppState, token: String, provider: Provider) {
    match provider {
        Provider::GitHub => {
            let mut guard = state.github_token.lock().unwrap();
            *guard = Some(token);
        }
        Provider::GitLab => {
            let mut guard = state.gitlab_token.lock().unwrap();
            *guard = Some(token);
        }
    }
}

pub fn get_token(state: &AppState, provider: Provider) -> Option<String> {
    match provider {
        Provider::GitHub => {
            let guard = state.github_token.lock().unwrap();
            guard.clone()
        }
        Provider::GitLab => {
            let guard = state.gitlab_token.lock().unwrap();
            guard.clone()
        }
    }
}

pub fn set_provider(state: &AppState, provider: Provider) {
    let mut guard = state.provider.lock().unwrap();
    *guard = provider;
}

pub fn get_provider(state: &AppState) -> Provider {
    let guard = state.provider.lock().unwrap();
    *guard
}

pub fn set_gitlab_url(state: &AppState, url: String) {
    let mut guard = state.gitlab_url.lock().unwrap();
    *guard = url;
}

pub fn get_gitlab_url(state: &AppState) -> String {
    let guard = state.gitlab_url.lock().unwrap();
    guard.clone()
}
