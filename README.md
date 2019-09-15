# JWordList

JWordList is a single-page web app for creating and maintaining a list of Japanese vocabulary. It uses Rust for the backend and for the frontend too, thanks to WASM!

To use it, download JMDict data (as gz) from https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project.

## Installation

Requirements:
* cargo

```bash
cd jwordlist
./build.sh --release
cd ..
cargo run --release
```

To watch the project:
* cargo-watch: `bash cargo install -f watch`

### Frontend
```bash
cd front
./build.sh --release --watch
```

### Backend
```bash
cargo watch -x 'run --release'
```

## LICENSE

MIT or Apache 2.0