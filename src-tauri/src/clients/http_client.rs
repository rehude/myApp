use reqwest::Client;

pub fn build_client() -> Client {
    Client::builder()
        .user_agent("tauri-git-client")
        .build()
        .unwrap()
}
