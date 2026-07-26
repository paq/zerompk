# tests

Tests for targets the default `cargo test` job does not cover.

## wasm32/

Run locally (requires node.js):

```sh
cd tests/wasm32
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
node tests.mjs target/wasm32-unknown-unknown/release/zerompk_wasm32_tests.wasm
```
