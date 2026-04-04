use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistedAuth {
    pub provider: Option<String>,
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
    pub gitlab_url: Option<String>,
}

fn get_auth_file_path() -> Option<PathBuf> {
    let auth_dir = dirs::data_local_dir()?.join("myapp");
    fs::create_dir_all(&auth_dir).ok()?;
    Some(auth_dir.join("auth.json"))
}

pub fn load_auth() -> PersistedAuth {
    let path = match get_auth_file_path() {
        Some(p) => p,
        None => return PersistedAuth::default(),
    };

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => PersistedAuth::default(),
    }
}

pub fn save_auth(auth: &PersistedAuth) -> Result<(), String> {
    let path = get_auth_file_path().ok_or("Failed to get auth file path")?;
    let contents = serde_json::to_string_pretty(auth).map_err(|e| e.to_string())?;
    fs::write(&path, contents).map_err(|e| e.to_string())
}
