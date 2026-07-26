// Test harness for the wasm32 skip_value overflow regression test.
//
// Usage: node tests.mjs <path-to.wasm>

import { readFileSync } from "node:fs";

const REJECTED = 0x0000dead; // must match lib.rs
const ACCEPTED = 0x0000acce; // must match lib.rs

const wasmPath = process.argv[2];
if (!wasmPath) {
  console.error("usage: node tests.mjs <path-to.wasm>");
  process.exit(2);
}

const bytes = readFileSync(wasmPath);
const { instance } = await WebAssembly.instantiate(bytes, {});
const ex = instance.exports;

function invoke(name) {
  try {
    return { ok: true, ret: ex[name]() >>> 0 };
  } catch (e) {
    return { ok: false, trap: `${e.constructor.name}: ${e.message}` };
  }
}

function hex(n) {
  return "0x" + (n >>> 0).toString(16).toUpperCase().padStart(8, "0");
}

// All checks
const checks = [
  { name: "skip_ext32_overflow", expect: REJECTED },
];

let failed = 0;
console.log("wasm32 regression test: skip_value ext32 overflow");
console.log("-------------------------------------------------");

for (const { name, expect } of checks) {
  const r = invoke(name);
  if (!r.ok) {
    failed++;
    console.log(`FAIL  ${name.padEnd(20)} trap: ${r.trap}`);
  } else if (r.ret !== expect) {
    failed++;
    console.log(`FAIL  ${name.padEnd(20)} got ${hex(r.ret)}, want ${hex(expect)}`);
    if (r.ret === ACCEPTED) {
      console.log(`      the truncated ext32 message was accepted (skip consumed 0 bytes)`);
    }
  } else {
    console.log(`ok    ${name.padEnd(20)} ${hex(r.ret)}`);
  }
}

console.log("-------------------------------------------------");
if (failed > 0) {
  console.log(`${failed} check(s) failed: the ext32 overflow is present on wasm32`);
  process.exit(1);
}
console.log("all checks passed: the malformed message was rejected");
