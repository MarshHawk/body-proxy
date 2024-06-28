# gets the rust nightly toolchain and support for wasm compilation
rust-toolchain:
	rustup toolchain install nightly
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack

# check wether compiled with wasm-pack
ifeq ($(shell test -e ./pkg/http_auth_bg.wasm && echo -n y),y)
	WASM_PATH=./pkg/body_proxy_bg.wasm
endif

build: clean
	wasm-pack build --release

clean:
	cargo clean
	rm -rf ./pkg