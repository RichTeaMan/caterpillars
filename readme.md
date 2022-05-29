# Caterpillars

Some creature experiments in a world of Rust. A web assembly version can be found at https://richteaman.github.io/caterpillars/.

## Running

Caterpillars can be run locally using Cargo:

```bash
cargo run
```

The web assembly version can also be run locally:

```bash
rustup target install wasm32-unknown-unknown && \
cargo install wasm-server-runner && \
cargo run --target wasm32-unknown-unknown
```