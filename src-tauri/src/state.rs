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

impl Provider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::GitHub => "github",
            Provider::GitLab => "gitlab",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "gitlab" => Provider::GitLab,
            _ => Provider::GitHub,
        }
    }
}

pub struct AppState {
    pub provider: Mutex<Provider>,
    pub current_token: Mutex<Option<String>>,
    pub gitlab_url: Mutex<String>,
}

impl AppState {
    pub fn new() -> Self {
        let data = crate::persist::load_data();

        // Find last used account
        let (provider, token, gitlab_url) = if let Some(last_id) = &data.last_used_id {
            if let Some(account) = data.accounts.iter().find(|a| &a.id == last_id) {
                let p = Provider::from_str(&account.provider);
                let url = account.gitlab_url.clone().unwrap_or_else(|| "https://gitlab.com".to_string());
                (p, Some(account.token.clone()), url)
            } else {
                (Provider::GitHub, None, "https://gitlab.com".to_string())
            }
        } else {
            (Provider::GitHub, None, "https://gitlab.com".to_string())
        };

        Self {
            provider: Mutex::new(provider),
            current_token: Mutex::new(token),
            gitlab_url: Mutex::new(gitlab_url),
        }
    }
}
