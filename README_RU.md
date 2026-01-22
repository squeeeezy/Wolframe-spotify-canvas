# Wolframe Spotify Canvas

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/wolframe-spotify-canvas.svg)](https://crates.io/crates/wolframe-spotify-canvas)
[![Docs.rs](https://docs.rs/wolframe-spotify-canvas/badge.svg)](https://docs.rs/wolframe-spotify-canvas)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/wolframe-spotify-canvas.svg)](https://crates.io/crates/wolframe-spotify-canvas)

**Rust-библиотека для получения Spotify Canvas (зацикленных видео) через внутренний GraphQL Pathfinder API.**

[English](README.md) | [Русский](README_RU.md)

</div>

Эта библиотека основана на реверс-инжиниринге внутреннего Spotify GraphQL Pathfinder API, который используется официальным веб-плеером. Она заменяет старые и нерабочие REST API эндпоинты (`canvaz-cache`).

---

## 🚀 Возможности

- **Pathfinder API:** Использует правильный эндпоинт `api-partner.spotify.com/pathfinder/v2` с Persisted Queries.
- **Автономный Client Token:** Автоматически запрашивает, генерирует и обновляет `client-token`, эмулируя сессию реального Web Player'а.
- **Гибкость:** Создана с расчётом на будущие обновления. Вы можете переопределить хеш GraphQL запроса и имя операции через конфиг.
- **Типизация:** Строгая обработка ошибок сети, токенов и изменений API.

## 📦 Установка

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
wolframe-spotify-canvas = "1.0.0"
tokio = { version = "1", features = ["full", "macros"] }
```

### Минимальная версия Rust (MSRV)

Для работы требуется Rust **1.83** или выше.

## ⚡ Использование

```rust
use wolframe_spotify_canvas::{CanvasClient, CanvasError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Получите валидный Spotify Access Token (Bearer)
    //    Можно взять из Network Tab веб-плеера Spotify (Authorization: Bearer ...)
    let access_token = env::var("SPOTIFY_TOKEN").expect("SPOTIFY_TOKEN not set");

    // 2. Инициализация клиента (поддерживает shared reqwest::Client)
    let mut client = CanvasClient::new();

    // 3. URI трека (например, "KORE" by Zynyx)
    let track_uri = "spotify:track:72Xn6x8xqegX64AKeJDsZt";

    println!("Получение canvas для: {}", track_uri);

    // 4. Запрос
    match client.get_canvas(track_uri, &access_token).await {
        Ok(canvas) => {
            println!("URL видео: {}", canvas.mp4_url);
        }
        Err(CanvasError::RateLimited { retry_after }) => {
            eprintln!("Rate limit! Повторите через {:?} мс", retry_after);
        }
        Err(e) => eprintln!("Ошибка: {}", e),
    }

    Ok(())
}
```

Смотрите `examples/simple.rs` для полного примера.

## 🔍 Observability

Библиотека использует [`tracing`](https://docs.rs/tracing) для структурированного логирования.

### Уровни логов

- `INFO` — Получение Canvas и успешные операции (по умолчанию)
- `DEBUG` — Внутренняя работа с токенами
- `TRACE` — Генерация Device ID, низкоуровневые детали

### Пример настройки

```rust
tracing_subscriber::fmt()
    .with_env_filter("wolframe_spotify_canvas=debug")
    .init();
```

### Интеграция с OpenTelemetry

Спаны автоматически передаются в системы распределённой трассировки (Jaeger, Datadog) при использовании `tracing-opentelemetry`.

---

## 🛠 Технические детали

Spotify отказался от простого REST API в пользу сложного GraphQL. Эта библиотека закрывает пропасть между ними.

### «Ловушка хеша»

GraphQL API Spotify не принимает «сырые» запросы. Он требует **Persisted Queries** — отправку SHA-256 хеша запроса.

- **Хеш по умолчанию:** `575138ab27cd5c1b3e54da54d0a7cc8d85485402de26340c2145f0f6bb5e7a9f` (зашит в библиотеку, но переопределяется через конфиг).
- **Operation Name:** `canvas` (важно: раньше использовался `getCanvas`, вызывавший ошибку 400).
- **Имя переменной:** `trackUri` (раньше было `uri`).

### Авторизация

Простого access-токена **недостаточно**. API требует заголовок `client-token`.

Библиотека эмулирует флоу `clienttoken.spotify.com`, генерируя случайный `device_id` и обменивая его на валидный временный `client-token`.

## 🤝 Вклад в проект

Pull request'ы приветствуются! Особенно если Spotify обновит хеш API — PR с обновлённым `DEFAULT_CANVAS_HASH` спасёт тысячи разработчиков.

## 📄 Лицензия

MIT License. См. [LICENSE](LICENSE).

---

*Сделано с ❤️ командой Wolframe.*
