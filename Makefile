all:
	cd js && npm install
	cd neon/native && npm install
	cd wasm && wasm-pack build --release --target nodejs
	npm install
	node test.js

.PHONY: all
