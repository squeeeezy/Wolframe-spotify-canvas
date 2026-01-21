# 🦀 wolframe-spotify-canvas

[![Crates.io](https://img.shields.io/crates/v/wolframe-spotify-canvas.svg)](https://crates.io/crates/wolframe-spotify-canvas)
[![Documentation](https://docs.rs/wolframe-spotify-canvas/badge.svg)](https://docs.rs/wolframe-spotify-canvas)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> **The modern standard for fetching Spotify Canvas (looping visuals) in 2026.**

This Rust library reverse-engineers the internal **Spotify GraphQL Pathfinder API** used by the official Spotify Web Player. It replaces the deprecated and non-functional REST API endpoints (`canvaz-cache`).

[🇷🇺 Read in Russian / Читать на русском](README_RU.md)

---

## 🚀 Features

-   **Pathfinder API:** Uses the correct `api-partner.spotify.com/pathfinder/v2` endpoint with Persisted Queries.
-   **Client Token Autonomy:** Automatically fetches, generates, and manages the required `client-token` (imitating a real Web Player session).
-   **Configurable:** Built to withstand frontend updates. You can override the Persisted Query Hash and Operation Name if Spotify changes them.
-   **Type-Safe:** Robust error handling for network failures, missing tokens, and API changes.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wolframe-spotify-canvas = "0.1.0"
tokio = { version = "1", features = ["full", "macros"] }
```

## ⚡ Usage

```rust
use wolframe_spotify_canvas::{CanvasClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize the client (handles client-token automatically)
    let mut client = CanvasClient::new();
    
    // 2. You need a valid Spotify Access Token (Bearer)
    // You can get this via SP_DC (librespot) or standard OAuth.
    let access_token = "YOUR_ACCESS_TOKEN";

    // 3. Fetch Canvas
    let track_uri = "spotify:track:4cOdK2wGLETKBW3PvgPWqT"; // Glimpse of Us
    
    match client.get_canvas(track_uri, access_token).await {
        Ok(canvas) => {
            println!("🎥 Canvas Found!");
            println!("MP4 URL: {}", canvas.mp4_url);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}
```

Check `examples/simple.rs` for a runnable example.

---

## 🛠 Technical Details (Reverse Engineering)

Spotify moved away from the simple `canvaz-cache` REST API to a complex GraphQL implementation. This library bridges that gap.

### The "Hash Trap"
Spotify's GraphQL API does not accept raw queries. It uses **Persisted Queries**, where the client sends a SHA-256 hash of the query.
*   **Default Hash:** `575138ab27cd5c1b3e54da54d0a7cc8d85485402de26340c2145f0f6bb5e7a9f` (Hardcoded default, but overridable).
*   **Operation Name:** `canvas` (Crucial distinction: older attempts used `getCanvas` which returns 400).
*   **Variable Name:** `trackUri` (Crucial distinction: formerly `uri`).

### Authentication
A standard access token is **not enough**. The API requires a `client-token` header.
*   This library emulates the flow of `clienttoken.spotify.com`, generating a random `device_id` and exchanging it for a valid, short-lived `client-token`.

## 🤝 Contributing

Contributions are welcome! Especially if Spotify updates their API hash—pull requests updating the `DEFAULT_CANVAS_HASH` are highly appreciated.

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.

---

*Built with ❤️ by the Wolframe Team.*
