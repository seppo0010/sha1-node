const wasm = require('./wasm')
const js = require('./js')
const Benchmark = require('benchmark');

var suite = new Benchmark.Suite;

wasm.ready.then(() => {
  const libs = ['wasm', 'js']
  libs.forEach(name => {
    suite.add(name + '#short', () => {
      const lib = require('./' + name)
      lib('Hello world')
    })
    suite.add(name + '#long', () => {
      const lib = require('./' + name)
      lib('Hello world'.repeat(123))
    })
  })
  suite
    .on('cycle', function(event) {
      console.log(String(event.target));
    })
    .run({ 'async': true });
})
