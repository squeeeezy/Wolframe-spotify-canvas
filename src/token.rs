use crate::error::{CanvasError, Result};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const CLIENT_TOKEN_URL: &str = "https://clienttoken.spotify.com/v1/clienttoken";
// This is the known client ID for the Spotify Web Player, widely used for this workaround.
const WEB_PLAYER_CLIENT_ID: &str = "d8a5ed958d274c2e8ee717e6a4b0971d";

#[derive(Debug, Clone)]
pub struct TokenManager {
    client_token: Option<String>,
    expires_at: Option<Instant>,
}

#[derive(Serialize)]
struct ClientTokenRequest {
    client_data: ClientData,
}

#[derive(Serialize)]
struct ClientData {
    client_version: String,
    client_id: String,
    js_sdk_data: JsSdkData,
}

#[derive(Serialize)]
struct JsSdkData {
    device_brand: String,
    device_model: String,
    os: String,
    os_version: String,
    device_id: String,
    device_type: String,
}

#[derive(Deserialize)]
struct ClientTokenResponse {
    granted_token: GrantedToken,
}

#[derive(Deserialize)]
struct GrantedToken {
    token: String,
    #[allow(dead_code)]
    expires_after_seconds: u64,
}

impl TokenManager {
    pub fn new() -> Self {
        Self { 
            client_token: None,
            expires_at: None,
        }
    }

    /// Fetches a new client-token from Spotify.
    /// This is required for the Pathfinder GraphQL API.
    #[tracing::instrument(level = "debug", skip(self, client), fields(expires_in = tracing::field::Empty))]
    pub async fn get_token(&mut self, client: &Client) -> Result<String> {
        if self.is_token_valid() {
             return Ok(self.client_token.as_ref().unwrap().clone());
        }

        tracing::debug!("Client token expired or missing, refreshing...");
        let device_id = generate_device_id();

        let request_body = ClientTokenRequest {
            client_data: ClientData {
                client_version: "1.2.42.432.g3121".to_string(), // Emulating a recent web client version
                client_id: WEB_PLAYER_CLIENT_ID.to_string(),
                js_sdk_data: JsSdkData {
                    device_brand: "Apple".to_string(),
                    device_model: "MacBookPro".to_string(),
                    os: "Mac OS".to_string(),
                    os_version: "10.15.7".to_string(),
                    device_id,
                    device_type: "computer".to_string(),
                },
            },
        };

        let response = client
            .post(CLIENT_TOKEN_URL)
            .json(&request_body)
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(CanvasError::TokenFetchFailed(format!(
                "Status: {}, Body: {}",
                status, text
            )));
        }

        let token_res: ClientTokenResponse = response.json().await?;
        
        self.client_token = Some(token_res.granted_token.token.clone());
        
        // Expire 60 seconds before actual expiry to be safe
        let secs = token_res.granted_token.expires_after_seconds.saturating_sub(60);
        self.expires_at = Some(Instant::now() + Duration::from_secs(secs));

        // Record the actual expiry into the current span
        tracing::Span::current().record("expires_in", token_res.granted_token.expires_after_seconds);
        tracing::debug!("Refreshed client-token");

        Ok(token_res.granted_token.token)
    }
    
    fn is_token_valid(&self) -> bool {
        match (&self.client_token, &self.expires_at) {
            (Some(_), Some(exp)) => Instant::now() < *exp,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn set_token(&mut self, token: String, expires_in: u64) {
        self.client_token = Some(token);
        self.expires_at = Some(Instant::now() + Duration::from_secs(expires_in.saturating_sub(60)));
    }
}

#[tracing::instrument(level = "trace")]
fn generate_device_id() -> String {
    let random_bytes: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen()).collect();
    hex::encode(random_bytes)
}
