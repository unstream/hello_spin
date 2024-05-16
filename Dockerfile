FROM scratch
COPY spin.toml /spin.toml
COPY target/wasm32-wasi/release/hello_spinf.wasm /target/wasm32-wasi/release/hello_spin.wasm
ENTRYPOINT ["/spin.toml"]
