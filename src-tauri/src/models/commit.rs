use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitSummary {
    pub sha: String,
    pub commit: CommitInfo,
    pub html_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitInfo {
    pub message: String,
    pub author: CommitAuthor,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitDetail {
    pub sha: String,
    pub commit: CommitInfo,
    pub html_url: String,
    pub files: Option<Vec<CommitFile>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitFile {
    pub filename: Option<String>,
    pub status: Option<String>,
    pub patch: Option<String>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
}
