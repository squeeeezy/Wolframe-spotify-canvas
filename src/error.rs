use thiserror::Error;

#[derive(Error, Debug)]
pub enum CanvasError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Failed to serialize/deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing access token. Please providing a valid SP_DC or access token.")]
    MissingAccessToken,

    #[error("Spotify API error: {0}")]
    SpotifyApi(String),

    #[error("Canvas not found for track: {0}")]
    NotFound(String),

    #[error("Token fetch failed: {0}")]
    TokenFetchFailed(String),
}

pub type Result<T> = std::result::Result<T, CanvasError>;
