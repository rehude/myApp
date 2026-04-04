use std::sync::Mutex;

pub struct AppState {
    pub github_token: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            github_token: Mutex::new(None),
        }
    }
}
