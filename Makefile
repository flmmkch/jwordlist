TARGET = debug
#TARGET = release
ifeq ($(TARGET), release)
	RELEASE_FLAG = --release
endif

all: front back
front: front-bindgen
front-bindgen: front-cargo
	cd front && wasm-bindgen --target web --out-dir ../static/dist/jwordlist target/wasm32-unknown-unknown/$(TARGET)/jwordlist_front.wasm
front-cargo:
	cd front && cargo build $(RELEASE_FLAG) --target wasm32-unknown-unknown
back:
	cargo build $(RELEASE_FLAG)
clean:
	cargo clean && cd front && cargo clean
.PHONY: clean back front-cargo front-bindgen
