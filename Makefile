all:
	cd js && npm install
	cd neon/native && cargo build --release
	cd wasm/sha1 && cargo build --release --target=wasm32-unknown-unknown
	node test.js

.PHONY: all
