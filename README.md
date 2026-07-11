# Snake

Snake is a fast, responsive, and secure traditional arcade Snake game built in Rust using Yew and WebAssembly on the frontend, served by a native Axum backend.

## Architecture and Stack

* Frontend: Yew (WASM)
* Backend: Axum (Rust) / Tokio
* Deployment: UBI container (Red Hat UBI9) on Docker Hub / Unraid / Podman / Docker Compose

## Code Map

The project is structured as a cargo workspace containing a Rust-based backend and a WebAssembly frontend built with Yew.

* [backend/src/main.rs](backend/src/main.rs): Application process entrypoint.
* [backend/src/bootstrap.rs](backend/src/bootstrap.rs): Runtime builder, state initialization, and application bootstrapper.
* [backend/src/config.rs](backend/src/config.rs): Configuration loader and validation.
* [backend/src/services/paths.rs](backend/src/services/paths.rs): Path resolution and environment variable directory overrides.
* [frontend/src/components/snake_game.rs](frontend/src/components/snake_game.rs): Main gameplay layout component handling keyboard input and grid viewport boundaries.
* [frontend/src/components/snake_logic.rs](frontend/src/components/snake_logic.rs): Pure game logic projecting movement vectors, snake growth, and self-collision checks.
* [frontend/src/components/snake/state.rs](frontend/src/components/snake/state.rs): Custom hook aggregating Yew states for centralized access.
* [frontend/src/components/snake/tick.rs](frontend/src/components/snake/tick.rs): Game tick interval registration and clock updates.
* [frontend/src/components/snake/food.rs](frontend/src/components/snake/food.rs): Spawning positions for normal food and managing Gold Food expirations.

## Key Features

* Traditional Arcade Loop: Classic gameplay with grid rendering, score tracking, and persistent high scores.
* Gold Food Mode: Flashing Gold Food that expires in 5 seconds (with a dynamic visual countdown bar) and awards +30 points.
* High Score Leaderboard: Persists the Top 10 player scores using simple file-based JSON storage (`leaderboard.json`).
* Sleek Neon Theme: Dark retro-futuristic styling matching the Super Metroid theme design system.
* Mobile-Friendly D-Pad: Integrated touch/D-Pad controls overlay for easy play on mobile and tablets.
* Access PIN Security: Lock down the interface with an optional numerical PIN for absolute privacy.

## Local Setup

Ensure you have the Rust toolchain (stable) and Trunk installed.

### Prerequisites

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk compiler
cargo install --locked trunk
```

### Development Commands

```bash
# 1. Run workspace tests
cargo test

# 2. Run clippy workspace checks
cargo clippy --workspace --all-targets

# 3. Start frontend Yew dev server (from frontend/)
cd frontend && trunk serve

# 4. Start backend Axum server (from backend/)
cd backend && cargo run
```

## Deployment and Hosting

Snake is optimized for self-hosting on Unraid, Docker, and Podman. Official images are built on Red Hat Universal Base Image (UBI9-minimal).

### Unraid Deployment Details

Snake templates are available through the community application repository.
* Docker Hub Repository: `ubermetroid/snake` (tags: `latest`, `ubi`, or version pins)
* Network Mode: Bridge (default port: `4501`)
* Volume Configuration: Mapped host folder to `/app/data` for leaderboard persistence (`leaderboard.json`).
* Security: The container runs with non-root privileges (`--user 99:100`). Ensure the mapped host path has appropriate read/write permissions for UID 99 and GID 100.

### Docker Compose

Create a `docker-compose.yml` file with the following service definition:

```yaml
services:
  snake:
    image: ubermetroid/snake:latest
    container_name: snake
    restart: unless-stopped
    volumes:
      - ${SNAKE_DATA_PATH:-./data}:/app/data
    ports:
      - ${PORT:-4501}:4501
    environment:
      PORT: 4501
      BASE_URL: ${BASE_URL:-http://localhost:4501}
      SNAKE_PIN: ${SNAKE_PIN:-}
      ALLOWED_ORIGINS: ${ALLOWED_ORIGINS:-*}
      MAX_ATTEMPTS: ${MAX_ATTEMPTS:-5}
      SITE_TITLE: ${SITE_TITLE:-Snake}
      ENABLE_TRANSLATION: ${ENABLE_TRANSLATION:-true}
      ENABLE_THEMES: ${ENABLE_THEMES:-true}
      ENABLE_PRINT: ${ENABLE_PRINT:-true}
      TZ: ${TZ:-UTC}
```

### Build UBI Image Locally

```bash
# From the repository root
podman build --format docker -f Containerfile.ubi \
  -t docker.io/ubermetroid/snake:1.0.43 \
  -t docker.io/ubermetroid/snake:latest \
  -t docker.io/ubermetroid/snake:ubi \
  .
```

## Configuration Options

| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server binds to inside the container. | `4501` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Snake` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies. | `http://localhost:4501` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). | `*` |
| `SNAKE_PIN` | Optional numerical PIN to lock access to the interface. | None |
| `SNAKE_DATA_DIR` | Directory where runtime state is persisted (`leaderboard.json`). | `./data` |
| `SNAKE_FRONTEND_DIR` | Path to the prebuilt Trunk SPA bundle. | `./frontend/dist` |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header. | `true` |
| `ENABLE_THEMES` | Enable the theme selector in the navigation header. | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header. | `true` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before rate lockout. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer. | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer. | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |

## License

Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 UberMetroid.
