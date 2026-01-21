use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CanvasError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Failed to serialize/deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Missing access token. Please provide a valid access token.")]
    MissingAccessToken,

    #[error("Spotify API error: {status} - {message}")]
    SpotifyApi { status: u16, message: String },

    #[error("Rate limited, retry after {retry_after:?}ms")]
    RateLimited { retry_after: Option<u64> },

    #[error("Token expired, refresh required")]
    TokenExpired,

    #[error("Canvas not found for track: {0}")]
    NotFound(String),

    #[error("Token fetch failed: {0}")]
    TokenFetchFailed(String),

    #[error("Hash outdated, update DEFAULT_CANVAS_HASH")]
    HashOutdated,

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, CanvasError>;
