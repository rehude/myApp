use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    #[serde(rename = "private")]
    pub is_private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub owner: Owner,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Owner {
    pub login: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoDetail {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    #[serde(rename = "private")]
    pub is_private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub language: Option<String>,
    pub owner: Owner,
}
