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
        // Load persisted auth
        let auth = crate::persist::load_auth();

        let provider = match auth.provider.as_deref() {
            Some("gitlab") => Provider::GitLab,
            _ => Provider::GitHub,
        };

        Self {
            provider: Mutex::new(provider),
            github_token: Mutex::new(auth.github_token),
            gitlab_token: Mutex::new(auth.gitlab_token),
            gitlab_url: Mutex::new(auth.gitlab_url.unwrap_or_else(|| "https://gitlab.com".to_string())),
        }
    }

    pub fn save_auth(&self) {
        let provider = {
            let guard = self.provider.lock().unwrap();
            match *guard {
                Provider::GitHub => "github",
                Provider::GitLab => "gitlab",
            }
        };

        let github_token = self.github_token.lock().unwrap().clone();
        let gitlab_token = self.gitlab_token.lock().unwrap().clone();
        let gitlab_url = self.gitlab_url.lock().unwrap().clone();

        let auth = crate::persist::PersistedAuth {
            provider: Some(provider.to_string()),
            github_token,
            gitlab_token,
            gitlab_url: Some(gitlab_url),
        };

        if let Err(e) = crate::persist::save_auth(&auth) {
            eprintln!("Failed to save auth: {}", e);
        }
    }
}
