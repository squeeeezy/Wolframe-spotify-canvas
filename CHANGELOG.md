# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2026-01-21

### Fixed
- **CI:** Pinned `idna` dependency to fix MSRV 1.75 checks.
- **Docs:** Fixed markdown formatting and removed dev suffixes in READMEs.
- **Style:** Applied `cargo fmt` to codebase.

## [1.0.0] - 2026-01-21

### Added
- **Shared Client:** `CanvasClient::with_client()` constructor to reuse an existing `reqwest::Client`.
- **Observability:** Full `tracing` instrumentation with `#[instrument]` and structured logging (INFO/DEBUG/TRACE levels).
- **Token Management:** Smart token TTL tracking. `TokenManager` now proactively refreshes the `client-token` before it expires.
- **Resilience:** Handling of `429 Too Many Requests` with `Retry-After` header parsing.
- **Error Handling:** `CanvasError` is now `#[non_exhaustive]` to allow future extensions without breaking changes.
- **CI/CD:** GitHub Actions workflow for automated testing and SemVer checks.
- **MSRV:** Minimum Supported Rust Version policy set to **1.75**.

### Changed
- **Breaking:** `get_canvas` signature now returns a Result with rich `CanvasError` enum instead of a generic error.
- **Dependency:** Updated dependencies to minimal compatible versions.

## [0.1.2] - 2026-01-21

### Fixed
- Added `Debug` and `Clone` derives to `CanvasClient` for better integration flexibility.
- Removed duplicate constants in consuming applications.

## [0.1.1] - 2026-01-21

### Fixed
- Corrected repository URL in `Cargo.toml`.

## [0.1.0] - 2026-01-21

### Added
- Initial release.
- Reverse-engineered Spotify GraphQL Pathfinder API for fetching Canvas videos.
- Automatic `client-token` generation via `clienttoken.spotify.com`.
- Customizable `CanvasConfig` (pathfinder URL, query hash).
