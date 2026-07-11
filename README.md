# Snake

Traditional arcade snake game.

## Quick Start

### Self-Hosting (Docker)
Pull and run the official Docker container:
```bash
docker run -d -p 4501:4501 -v /path/to/appdata:/app/data ubermetroid/snake:latest
```

### Local Development
To run locally, ensure you have Rust and Cargo installed:
```bash
cargo run --bin server
```
