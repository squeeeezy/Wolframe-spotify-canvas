use std::env;
use wolframe_spotify_canvas::{CanvasClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Get Access Token from environment or args
    let access_token = match env::var("SPOTIFY_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("⚠️  SPOTIFY_TOKEN environment variable not set.");
            eprintln!("   Please set it to a valid Spotify Access Token.");
            eprintln!("   ");
            eprintln!("   HOW TO GET A TOKEN:");
            eprintln!("   1. Open https://open.spotify.com in your browser.");
            eprintln!("   2. Log in.");
            eprintln!("   3. Open Developer Tools (F12) -> Network tab.");
            eprintln!("   4. Filter for 'pathfinder'.");
            eprintln!("   5. Click any request, go to 'Headers' -> 'Request Headers'.");
            eprintln!("   6. Copy the 'authorization' header (starts with 'Bearer ...').");
            eprintln!(
                "   7. Run: $env:SPOTIFY_TOKEN='...' ; cargo run --example simple (PowerShell)"
            );
            return Ok(());
        }
    };

    // 2. Initialize Client
    let mut client = CanvasClient::new();

    // 3. Define a track URI (e.g., "KORE" by Zynyx)
    let track_uri = "spotify:track:72Xn6x8xqegX64AKeJDsZt";

    println!("Fetching canvas for: {}", track_uri);

    // 4. Fetch
    match client.get_canvas(track_uri, &access_token).await {
        Ok(canvas) => {
            println!("✅ Canvas Found!");
            println!("   - MP4 URL: {}", canvas.mp4_url);
            println!("   - Canvas URI: {:?}", canvas.uri);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    Ok(())
}
