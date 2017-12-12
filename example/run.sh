(cd .. && python -m SimpleHTTPServer 8000 &)
CARGO_INCREMENTAL=0 cargo watch -x 'build --target=wasm32-unknown-unknown --release'