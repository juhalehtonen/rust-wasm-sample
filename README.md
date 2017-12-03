# Rust + WebAssembly example
An as-minimal-as-it-gets sample of running WebAssembly in your browser. Compiled from Rust using the nightly build that does not need any additional tools anymore :-)  

## Building
Currently you need Rust Nightly to compile without emscripten:
```
rustc +nightly --target wasm32-unknown-unknown -O src/main.rs
```

## Usage
See `index.html` and `src/main.rs` for details.
