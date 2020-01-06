
const path = require('path').join(__dirname, 'jess_bg.wasm');
const bytes = require('fs').readFileSync(path);
let imports = {};
imports['./jess.js'] = require('./jess.js');

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
