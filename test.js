const wasm = require('./wasm/pkg').digest
const js = require('./js')
const neon = require('./neon')
const Benchmark = require('benchmark');

var suite = new Benchmark.Suite;


const libs = ['wasm', 'js', 'neon']
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
  .on('cycle', function (event) {
    console.log(String(event.target));
  })
  .run({ 'async': true });

