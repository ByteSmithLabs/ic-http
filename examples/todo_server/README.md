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

### Local endpoint

`GET /ping`

```
curl -s \
    "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/ping" \
    --resolve "$(dfx canister id http_server).localhost:$(dfx info webserver-port):127.0.0.1"
```

`POST /hello`

```
curl -s -X POST \
    "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/hello" \
    --resolve "$(dfx canister id http_server).localhost:$(dfx info webserver-port):127.0.0.1" \
    -H "Content-Type: application/json" \
    -d '{ "title": "Learn Motoko" }' | jq
```

### Mainnet endpoint

```
curl -s https://5wbji-niaaa-aaaab-aaelq-cai.icp0.io/ping | jq
```
