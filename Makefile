all: front back
front: front-bindgen
front-bindgen: front-cargo
	cd front && wasm-bindgen --target web --out-dir ../static/dist/jwordlist target/wasm32-unknown-unknown/$TARGET_SUBDIR/jwordlist_front.wasm
front-cargo:
	cd front && cargo build --release --target wasm32-unknown-unknown
back:
	cargo build --release
clean:
	cargo clean && cd front && cargo clean
.PHONY: clean back front-cargo front-bindgen
