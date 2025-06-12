# ic-http

A lightweight Rust HTTP server library designed for Internet Computer (IC) canisters with built-in routing and request handling capabilities.

## Features

- Simple HTTP server implementation optimized for IC canisters
- Flexible routing system supporting all standard HTTP methods
- Path-based request routing with fallback handling
- Built on `ic-http-certification` for IC compatibility
- Async request handling with futures support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ic-http = { git = "https://github.com/ByteSmithLabs/ic-http.git", branch = "main"}
```

## Quick Start

```rust
use ic_http::{Server, Handler};
use ic_http_certification::{HttpRequest, HttpResponse};
use std::pin::Pin;
use std::future::Future;

// Create a new server
let mut server = Server::new();

// Define a handler function
let hello_handler: Handler = |_req: &HttpRequest<'static>| {
    Box::pin(async {
        HttpResponse::ok(
            b"Hello, World!",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    })
};

// Register routes
server.route("GET", "/hello", hello_handler);

// Handle incoming requests
let response = server.handle(&request).await;
```

## Multiple Routes Example

```rust
use ic_http::{Server, Handler};
use ic_http_certification::{HttpRequest, HttpResponse};

let mut server = Server::new();

// GET route
let get_users: Handler = |_req| {
    Box::pin(async {
        HttpResponse::ok(
            b"[{\"id\": 1, \"name\": \"Alice\"}]",
            vec![("Content-Type".into(), "application/json".into())],
        )
        .build()
    })
};

// POST route
let create_user: Handler = |_req| {
    Box::pin(async {
        HttpResponse::ok(
            b"{\"id\": 2, \"name\": \"Bob\"}",
            vec![("Content-Type".into(), "application/json".into())],
        )
        .build()
    })
};

// Register routes
server.route("GET", "/users", get_users);
server.route("POST", "/users", create_user);

// Custom fallback for unmatched routes
server.with_fallback(|_req| {
    Box::pin(async {
        HttpResponse::not_found(
            b"Custom 404 - Route not found",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    })
});
```

## API Overview

### Server

The main `Server` struct provides the core functionality:

```rust
impl Server {
    // Create a new server instance
    pub fn new() -> Self;

    // Register a route with method, path, and handler
    pub fn route(&mut self, method: &str, path: &str, handler: Handler);

    // Set a custom fallback handler for unmatched routes
    pub fn with_fallback(&mut self, handler: Handler);

    // Handle an incoming HTTP request
    pub async fn handle(&self, req: &HttpRequest<'static>) -> HttpResponse<'static>;

    // Check if a string is a valid HTTP method
    pub fn is_http_method(method: &str) -> bool;
}
```

### Handler Type

Handlers are async functions that process requests:

```rust
pub type Handler = for<'a> fn(
    &'a HttpRequest<'static>,
) -> Pin<Box<dyn Future<Output = HttpResponse<'static>> + 'a>>;
```

### Supported HTTP Methods

The server supports all standard HTTP methods:

- GET, POST, PUT, DELETE
- PATCH, OPTIONS, HEAD
- TRACE, CONNECT

## Usage in IC Canisters

Here's how to integrate the HTTP server into an Internet Computer canister:

```rust
use ic_cdk::query;
use ic_http::{Server, Handler};
use ic_http_certification::{HttpRequest, HttpResponse};
use std::cell::RefCell;

thread_local! {
    static HTTP_SERVER: RefCell<Server> = RefCell::new({
        let mut server = Server::new();

        let health_check: Handler = |_req| {
            Box::pin(async {
                HttpResponse::ok(
                    b"OK",
                    vec![("Content-Type".into(), "text/plain".into())],
                )
                .build()
            })
        };

        server.route("GET", "/health", health_check);
        server
    });
}

#[query]
async fn http_request(req: HttpRequest) -> HttpResponse {
    HTTP_SERVER.with(|server| {
        let server = server.borrow();
        server.handle(&req)
    }).await
}
```

## Development

### Building

```sh
cargo build
```

### Testing

```sh
cargo test
```

### Documentation

Generate and view the documentation:

```sh
cargo doc --open
```

## Dependencies

This library builds on several key dependencies:

- `ic-http-certification`: IC-specific HTTP types and certification
- `matchit`: Fast path-based router
- `serde`: Serialization framework
- `futures`: Async programming utilities

## Examples

Check out the `examples/` directory in the repository for complete working examples, including:

- Basic HTTP server setup
- IC canister integration
- Custom route handlers

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under MIT License
