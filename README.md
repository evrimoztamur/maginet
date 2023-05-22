Watch `server/src` and `shared` for server-related source changes, and rerun server:
```watchexec -w server/src -w shared -r -e rs -- cargo run -p server```

Watch `src` and `shared` for client-related source changes, and rebuild deployable:
```watchexec -w src -w shared -r -e rs -- wasm-pack build --target web --debug --out-name maginet_aee75fc --out-dir static/js/pkg -- --features deploy```

If running locally via the tunnel, do not enable the `deploy` feature:
```watchexec -w src -w shared -r -e rs -- wasm-pack build --target web --debug --out-name maginet_aee75fc --out-dir static/js/pkg```