use crate::error::{CanvasError, Result};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const CLIENT_TOKEN_URL: &str = "https://clienttoken.spotify.com/v1/clienttoken";
// This is the known client ID for the Spotify Web Player, widely used for this workaround.
const WEB_PLAYER_CLIENT_ID: &str = "d8a5ed958d274c2e8ee717e6a4b0971d";

#[derive(Debug, Clone)]
pub struct TokenManager {
    client_token: Option<String>,
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
    expires_after_seconds: i64,
}

impl TokenManager {
    pub fn new() -> Self {
        Self { client_token: None }
    }

    /// Fetches a new client-token from Spotify.
    /// This is required for the Pathfinder GraphQL API.
    pub async fn get_token(&mut self, client: &Client) -> Result<String> {
        if let Some(token) = &self.client_token {
            // In a real implementation, check expiry. For now, we return cached if present.
            // TODO: Implement expiry check
            return Ok(token.clone());
        }

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

        Ok(token_res.granted_token.token)
    }
    
    #[allow(dead_code)]
    pub fn set_token(&mut self, token: String) {
        self.client_token = Some(token);
    }
}

fn generate_device_id() -> String {
    let random_bytes: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen()).collect();
    hex::encode(random_bytes)
}
