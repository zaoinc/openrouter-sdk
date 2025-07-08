use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenRouterError {
    #[error("API request failed: {0}")]
    ApiError(String),
    
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}