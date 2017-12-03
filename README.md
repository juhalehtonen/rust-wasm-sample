# Rust + WebAssembly example
Currently you need Rust Nightly to compile without emscripten:
```
rustc +nightly --target wasm32-unknown-unknown -O src/main.rs
```

## Usage
See `index.html` and `src/main.rs` for details.
