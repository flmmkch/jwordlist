TARGET = debug
#TARGET = release
ifeq ($(TARGET), release)
	RELEASE_FLAG = --release
endif

all: front back
front: front-bindgen
front-bindgen: front-cargo
	wasm-bindgen --target web --out-dir static/dist/jwordlist target/wasm32-unknown-unknown/$(TARGET)/jwordlist_front.wasm
front-cargo:
	cargo build -p jwordlist-front $(RELEASE_FLAG) --target wasm32-unknown-unknown
back:
	cargo build -p jwordlist $(RELEASE_FLAG)
clean:
	cargo clean
.PHONY: clean back front-cargo front-bindgen
