all:
	cd js && npm install
	cd neon/native && npm install
	cd wasm/sha1 && cargo build --release --target=wasm32-unknown-unknown
	npm install
	node test.js

.PHONY: all
