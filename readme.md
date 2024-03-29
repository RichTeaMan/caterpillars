# Caterpillars

Some creature experiments in a world of Rust. A web assembly version can be found at https://richteaman.github.io/caterpillars/.

## Running

Caterpillars can be run locally using Cargo:

```bash
cargo run
```

Specfic configurations can be run with the JSON file name as a parameter:

```bash
cargo run -- caterpillar.json
```

The web assembly version can also be run locally:

```bash
rustup target install wasm32-unknown-unknown && \
cargo install wasm-server-runner && \
cargo run --target wasm32-unknown-unknown
```

### Building WASM

```bash
rustup target install wasm32-unknown-unknown && cargo install wasm-bindgen-cli && \
cargo build --all-features --target wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./docs/ --target web target/wasm32-unknown-unknown/debug/caterpillars.wasm && \
cp assets/ docs/. -r
```

Local HTTP server:

```bash
python3 -m http.server
```

Note that this doesn't work so well in Firefox due to HTTPS constraints. Chrome has a better time of it.
