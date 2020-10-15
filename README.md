# sha1-node

Benchmark to compare sha1 implementation in javascript, web assembly and neon.

Results: (Updated 10-12-2020)

```
wasm#short x 229,630 ops/sec ±0.47% (85 runs sampled)
wasm#long x 114,446 ops/sec ±0.49% (89 runs sampled)
js#short x 275,967 ops/sec ±0.70% (90 runs sampled)
js#long x 20,944 ops/sec ±0.63% (89 runs sampled)
neon#short x 691,628 ops/sec ±0.35% (94 runs sampled)
neon#long x 188,252 ops/sec ±0.39% (91 runs sampled)
```

## Prerequisites

* nodejs
* rust with wasm32-unknown-unknown target

## Running the project 

Run `make` to run the benchmark.

## Libraries used

### js

Using the npmjs sha1 package

https://www.npmjs.com/package/sha1

This package has about 50.000 weekly downloads as of today.

### neon

Rust bindings for writing safe and fast native Node.js modules.

https://www.neon-bindings.com/

For sha1 I used the cargo package

https://crates.io/crates/sha1

### wasm

WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine.

https://webassembly.org/

For sha1 I used the cargo package

https://crates.io/crates/sha1
