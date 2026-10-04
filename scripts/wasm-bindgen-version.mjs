import { readFileSync } from 'node:fs';
const lock = readFileSync(new URL('../Cargo.lock', import.meta.url), 'utf8');
const version = lock.match(/name = "wasm-bindgen"\nversion = "([^"]+)"/)?.[1];
if (!version) throw new Error('wasm-bindgen is missing from Cargo.lock.');
console.log(version);
