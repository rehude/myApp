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

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub code: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            code: None,
        }
    }

    pub fn error(message: String, code: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            code: Some(code.to_string()),
        }
    }

    pub fn unauthorized(message: String) -> Self {
        Self::error(message, "UNAUTHORIZED")
    }

    pub fn network_error(message: String) -> Self {
        Self::error(message, "NETWORK_ERROR")
    }

    pub fn not_logged_in() -> Self {
        Self::error("Not logged in".to_string(), "NOT_LOGGED_IN")
    }

    pub fn unknown(message: String) -> Self {
        Self::error(message, "UNKNOWN")
    }
}

impl<T> From<AppError> for ApiResponse<T> {
    fn from(e: AppError) -> Self {
        match e {
            AppError::Unauthorized => ApiResponse::unauthorized(e.to_string()),
            AppError::NotLoggedIn => ApiResponse::not_logged_in(),
            AppError::Network(msg) => ApiResponse::network_error(msg),
            AppError::Unknown(msg) => ApiResponse::unknown(msg),
        }
    }
}
