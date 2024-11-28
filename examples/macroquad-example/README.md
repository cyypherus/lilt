# Macroquad example

# Build wasm

In workspace root :

```bash
cargo build -p macroquad-example --target wasm32-unknown-unknown

# Copy built wasm file
cp target/wasm32-unknown-unknown/debug/macroquad-example.wasm -o examples/macroquad-example/main.wasm

# Serve web app (here with rust's basic-http-server)
basic-http-server examples/macroquad-example
```
