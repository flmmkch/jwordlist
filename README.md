# JWordList

JWordList is a single-page web app for creating and maintaining a list of Japanese vocabulary. It uses Rust for the backend and for the frontend too, thanks to WASM!

To use it, download JMDict data (as gz) from https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project.

## Installation

Requirements:
* cargo to compile Rust: https://rustup.rs/
* the wasm32-unknown-unknown target for Rust webassembly support: `rustup target add wasm32-unknown-unknown`
* wasm-bindgen-cli for Rust webassembly support: `cargo install -f wasm-bindgen-cli`

### Compiling

Simply use the Makefile provided in the repository to compile the project.

```bash
make TARGET=release
```

### Running

* Edit **jwordlist.yaml** according to your needs.
* Download the dictionary file (as a .gz) from https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project to the path specified in **jwordlist.yaml**

```bash
cargo run --release -p jwordlist
```

### Debug mode with source file watching

* cargo-watch: `bash cargo install -f watch`

```
cd front
cargo watch -x 'run -p jwordlist'
```
and
```bash
./build-front.sh --watch
```

## LICENSE

MIT or Apache 2.0
