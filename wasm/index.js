const bundle = require('./bundle')
const path = require('path')

const Module = {}
module.exports = function(str) {
  let buf = bundle.newString(Module, str);
  let outptr = Module.digest(buf);
  let result = bundle.copyCStr(Module, outptr);
  Module.dealloc_str(buf);
  return result;
}

module.exports.ready = bundle.fetchAndInstantiate(path.dirname(__filename) + "/sha1/target/wasm32-unknown-unknown/release/sha1_digest.wasm", {})
.then(mod => {
  Module.alloc   = mod.exports.alloc;
  Module.dealloc = mod.exports.dealloc;
  Module.digest  = mod.exports.digest;
  Module.memory  = mod.exports.memory;
  Module.dealloc_str  = mod.exports.dealloc_str;
});
