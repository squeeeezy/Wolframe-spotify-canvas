mod error;
mod token;

pub use error::{CanvasError, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use token::TokenManager;

/// Default GraphQL endpoint for Spotify Pathfinder API.
pub const DEFAULT_PATHFINDER_URL: &str = "https://api-partner.spotify.com/pathfinder/v2/query";
/// Known working hash for the Canvas operation (as of Jan 2026).
pub const DEFAULT_CANVAS_HASH: &str = "575138ab27cd5c1b3e54da54d0a7cc8d85485402de26340c2145f0f6bb5e7a9f";
pub const DEFAULT_OPERATION_NAME: &str = "canvas";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    /// Direct URL to the MP4 video file.
    pub mp4_url: String,
    /// The Spotify URI of the canvas.
    pub uri: Option<String>,
    /// The Track URI mismatch warning (if any).
    pub track_uri: String,
}

#[derive(Debug, Clone)]
pub struct CanvasConfig {
    pub pathfinder_url: String,
    pub operation_name: String,
    pub query_hash: String,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            pathfinder_url: DEFAULT_PATHFINDER_URL.to_string(),
            operation_name: DEFAULT_OPERATION_NAME.to_string(),
            query_hash: DEFAULT_CANVAS_HASH.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CanvasClient {
    http: reqwest::Client,
    token_manager: TokenManager,
    config: CanvasConfig,
}

impl CanvasClient {
    pub fn new() -> Self {
        Self::with_config(CanvasConfig::default())
    }

    pub fn with_config(config: CanvasConfig) -> Self {
        Self {
            http: reqwest::Client::new(),
            token_manager: TokenManager::new(),
            config,
        }
    }

    /// Fetch the Canvas video URL for a given Spotify Track URI.
    ///
    /// # Arguments
    ///
    /// * `track_uri` - The Spotify Track URI (e.g., "spotify:track:...")
    /// * `access_token` - A valid Spotify Access Token (Bearer).
    pub async fn get_canvas(&mut self, track_uri: &str, access_token: &str) -> Result<Canvas> {
        // 1. Ensure we have a valid client-token
        let client_token = self.token_manager.get_token(&self.http).await?;

        // 2. Prepare headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))
                .map_err(|e| CanvasError::SpotifyApi(format!("Invalid access token: {}", e)))?,
        );
        headers.insert(
            "client-token",
            HeaderValue::from_str(&client_token)
                .map_err(|e| CanvasError::SpotifyApi(format!("Invalid client token: {}", e)))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // 3. Prepare GraphQL body
        let request_body = json!({
            "operationName": self.config.operation_name,
            "variables": {
                "trackUri": track_uri,
            },
            "extensions": {
                "persistedQuery": {
                    "version": 1,
                    "sha256Hash": self.config.query_hash
                }
            }
        });

        // 4. Send Request
        let response = self
            .http
            .post(&self.config.pathfinder_url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            // Handle specific cases like Token Expired if possible, for now general error
            return Err(CanvasError::SpotifyApi(format!(
                "GraphQL request failed ({}): {}",
                status, text
            )));
        }

        let body_text = response.text().await?;
        
        // 5. Parse Response
        let graph_response: GraphResponse = serde_json::from_str(&body_text)?;

        // Deep matching to extract URL
        let canvas_data = graph_response
            .data
            .and_then(|d| d.track_union)
            .and_then(|t| t.canvas);

        match canvas_data {
            Some(cd) => {
                if let Some(url) = cd.url {
                     Ok(Canvas {
                        mp4_url: url,
                        uri: cd.uri,
                        track_uri: track_uri.to_string(),
                    })
                } else if let Some(_uri) = cd.uri {
                     // Sometimes only URI is returned? Rare but possible fallback logic could go here if we knew how to resolve it.
                     // For now, treat as not found if no URL.
                     Err(CanvasError::NotFound(track_uri.to_string()))
                } else {
                     Err(CanvasError::NotFound(track_uri.to_string()))
                }
            }
            None => Err(CanvasError::NotFound(track_uri.to_string())),
        }
    }
}

// Internal GraphQL Response Structures

#[derive(Deserialize)]
struct GraphResponse {
    data: Option<GraphData>,
    #[allow(dead_code)]
    errors: Option<Vec<GraphError>>,
}

#[derive(Deserialize)]
struct GraphData {
    #[serde(rename = "trackUnion")]
    track_union: Option<TrackUnion>,
}

#[derive(Deserialize)]
struct TrackUnion {
    canvas: Option<CanvasData>,
}

#[derive(Deserialize)]
struct CanvasData {
    url: Option<String>,
    uri: Option<String>,
}

#[derive(Deserialize)]
struct GraphError {
    #[allow(dead_code)]
    message: String,
}
