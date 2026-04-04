
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not logged in")]
    NotLoggedIn,
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Network(e.to_string())
    }
}
