use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Provider {
    GitHub,
    GitLab,
}

impl Default for Provider {
    fn default() -> Self {
        Provider::GitHub
    }
}

pub struct AppState {
    pub provider: Mutex<Provider>,
    pub github_token: Mutex<Option<String>>,
    pub gitlab_token: Mutex<Option<String>>,
    pub gitlab_url: Mutex<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            provider: Mutex::new(Provider::GitHub),
            github_token: Mutex::new(None),
            gitlab_token: Mutex::new(None),
            gitlab_url: Mutex::new("https://gitlab.com".to_string()),
        }
    }
}
