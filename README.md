# 💬 rust_xat

A simple Chat App written in Rust.

## Usage

Setup an `.env` file and run:

```bash
# start redis
docker-compose up -d

# start server
cargo run

# hot reloading
cargo-watch -c -q -w . -x "run"
```