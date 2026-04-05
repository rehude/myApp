use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub provider: String,
    pub token: String,
    pub gitlab_url: Option<String>,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistedData {
    pub accounts: Vec<Account>,
    pub last_used_id: Option<String>,
}

fn get_auth_file_path() -> Option<PathBuf> {
    let auth_dir = dirs::data_local_dir()?.join("myapp");
    fs::create_dir_all(&auth_dir).ok()?;
    Some(auth_dir.join("accounts.json"))
}

pub fn load_data() -> PersistedData {
    let path = match get_auth_file_path() {
        Some(p) => p,
        None => return PersistedData::default(),
    };

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => PersistedData::default(),
    }
}

pub fn save_data(data: &PersistedData) -> Result<(), String> {
    let path = get_auth_file_path().ok_or("Failed to get auth file path")?;
    let contents = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, contents).map_err(|e| e.to_string())
}
