pub mod state;
pub mod error;
pub mod models;
pub mod clients;
pub mod services;
pub mod commands;
pub mod providers;
pub mod persist;

use state::AppState;
use commands::{
    init_app, logout, save_account, get_accounts, delete_account,
    set_current_account, get_current_state, set_provider, get_repos, get_repo_detail,
    get_commits, get_commit_detail
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            init_app,
            logout,
            save_account,
            get_accounts,
            delete_account,
            set_current_account,
            get_current_state,
            set_provider, 
            get_repos,
            get_repo_detail,
            get_commits,
            get_commit_detail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
