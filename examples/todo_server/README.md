# `http_server_example`

### Build

```bash
# Build the package
cargo build --release --target wasm32-unknown-unknown --package http_server --locked

# Extract candid interface
candid-extractor "../../target/wasm32-unknown-unknown/release/http_server.wasm" > "http_server.did"
```

### Deploy

```
dfx deploy --playground
```

### Usage

**CLI**

```bash
# GET request
curl "https://<CANISTER_ID>.icp0.io/ping"

# POST request
curl -X POST "https://<CANISTER_ID>.icp0.io/ping" \
  -H "Content-Type: application/json" \
  -d '{"key":"value"}'
```

```
https://<CANISTER_ID>.icp0.io/hello
```

### Local endpoint

```
curl -s \
    "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/hello"
```
