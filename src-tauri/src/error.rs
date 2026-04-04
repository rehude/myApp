
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network error")]
    Network,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not logged in")]
    NotLoggedIn,
    #[error("Unknown error")]
    Unknown,
}

impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        AppError::Network
    }
}
