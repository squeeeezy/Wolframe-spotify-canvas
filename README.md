# Wolframe Spotify Canvas (v1.0.0-dev)

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/wolframe-spotify-canvas.svg)](https://crates.io/crates/wolframe-spotify-canvas)
[![Docs.rs](https://docs.rs/wolframe-spotify-canvas/badge.svg)](https://docs.rs/wolframe-spotify-canvas)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/wolframe-spotify-canvas.svg)](https://crates.io/crates/wolframe-spotify-canvas)

**Rust library for fetching Spotify Canvas (looping visuals) via the internal GraphQL Pathfinder API.**

[English](README.md) | [Русский](README_RU.md)

</div>
This Rust library reverse-engineers the internal **Spotify GraphQL Pathfinder API** used by the official Spotify Web Player. It replaces the deprecated and non-functional REST API endpoints (`canvaz-cache`).

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
wolframe-spotify-canvas = "1.0.0"
tokio = { version = "1", features = ["full", "macros"] }
```

### Minimum Supported Rust Version (MSRV)
This crate requires Rust 1.75 or newer.



## ⚡ Usage

```rust
use wolframe_spotify_canvas::{CanvasClient, CanvasError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get a valid Spotify Access Token (Bearer)
    //    You can grab this from the Spotify Web Player network tab (Authorization: Bearer ...)
    let access_token = env::var("SPOTIFY_TOKEN").expect("SPOTIFY_TOKEN not set");

    // 2. Initialize the client (now supports shared reqwest::Client if needed)
    let mut client = CanvasClient::new();

    // 3. Define a track URI (e.g., "KORE" by Zynyx)
    let track_uri = "spotify:track:72Xn6x8xqegX64AKeJDsZt";

    println!("Fetching canvas for: {}", track_uri);

    // 4. Fetch the canvas
    match client.get_canvas(track_uri, &access_token).await {
        Ok(canvas) => {
            println!("Canvas URL: {}", canvas.mp4_url);
        }
        Err(CanvasError::RateLimited { retry_after }) => {
            eprintln!("Rate limited! Retry after {:?}ms", retry_after);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```

Check `examples/simple.rs` for a runnable example.

## 🔍 Observability

This crate uses [`tracing`](https://docs.rs/tracing) for structured logging.

### Log Levels
- `INFO` — Canvas fetch operations (default)
- `DEBUG` — Token management internals
- `TRACE` — Device ID generation, low-level details

### Example Setup
```rust
tracing_subscriber::fmt()
    .with_env_filter("wolframe_spotify_canvas=debug")
    .init();
```

### OpenTelemetry Integration
Spans are automatically propagated to distributed tracing backends (Jaeger, Datadog) when using `tracing-opentelemetry`.

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
