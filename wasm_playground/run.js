
// read the first command line argument as the file name
const filename = process.argv[2];

// Runs the add function from add.wasm
const fs = require('fs');
const buf = fs.readFileSync(`${filename}.wasm`);
const mod = new WebAssembly.Module(new Uint8Array(buf));
const global = new WebAssembly.Global({ value: 'i32', mutable: true }, 10);
const memory = new WebAssembly.Memory({ initial: 1 });

const wasmLog = (offset, length) => {
	const bytes = new Uint8Array(memory.buffer, offset, length);
	const string = new TextDecoder('utf8').decode(bytes);
	console.log(string);
};

const wasmLogRaw = (offset, length) => {
	const bytes = new Uint8Array(memory.buffer, offset, length);
	console.log(bytes);
};

const instance = new WebAssembly.Instance(mod,
	{ js: { global, mem: memory }, console: { log: wasmLog, logRaw: wasmLogRaw } });
console.log(instance.exports.main());
