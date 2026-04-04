pub mod state;
pub mod error;
pub mod models;
pub mod clients;
pub mod services;
pub mod commands;
pub mod providers;

use state::AppState;
use commands::{set_provider, set_github_token, set_gitlab_token, set_gitlab_url, get_repos, get_repo_detail, get_commits, get_commit_detail};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            set_provider,
            set_github_token,
            set_gitlab_token,
            set_gitlab_url,
            get_repos,
            get_repo_detail,
            get_commits,
            get_commit_detail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
