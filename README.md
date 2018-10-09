# sha1-node

Benchmark to compare sha1 implementation in javascript, web assembly and neon.

Results on my laptop

```
wasm#short x 30,503 ops/sec ±4.48% (68 runs sampled)
wasm#long x 17,856 ops/sec ±2.82% (78 runs sampled)
js#short x 123,116 ops/sec ±1.96% (87 runs sampled)
js#long x 10,177 ops/sec ±2.75% (81 runs sampled)
neon#short x 214,933 ops/sec ±1.65% (84 runs sampled)
neon#long x 68,124 ops/sec ±1.64% (85 runs sampled)
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
