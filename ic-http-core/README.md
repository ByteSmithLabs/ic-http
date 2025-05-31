# ic-http

A Rust HTTP library with built-in documentation for common operations and a flexible HTTP server framework.

## Features

- Simple HTTP server implementation with Express.js/Axum-like API
- Routing system for HTTP requests (GET, POST, PUT, DELETE, etc.)
- Request and Response abstractions
- JSON support for requests and responses
- Comprehensive documentation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ic-http = "0.1.0"
```

## HTTP Server Usage

```rust
use ic_http::{SimpleHttpServer, HttpServer, Request, Response};
use std::collections::HashMap;

fn main() {
    // Create a new HTTP server
    let mut server = SimpleHttpServer::new();

    // Register routes
    server.get("/hello", |_req| {
        Box::pin(async move {
            Response::text("Hello, World!", 200)
        })
    });

    server.post("/api/users", |req| {
        Box::pin(async move {
            match req.json::<serde_json::Value>() {
                Ok(json) => Response::json(&json, 201).unwrap_or_else(|_| {
                    Response::text("Error creating JSON response", 500)
                }),
                Err(_) => Response::text("Invalid JSON", 400),
            }
        })
    });

    // Example of handling a request
    let request = Request::new(
        ic_http::Method::GET,
        "/hello".to_string(),
        HashMap::new(),
        Vec::new()
    );

    // In a real application, you would run this in an async runtime
    let future_response = server.handle_request(request);

    // ...
}
```

## Router Usage

You can use the built-in `Router` to register exact and prefix-based routes in your canister.

```rust
use ic_http::{Router};
use ic_http::{HttpRequest, HttpResponse};

// Example handler functions
fn handle_hello(req: HttpRequest) -> HttpResponse {
    HttpResponseBuilder::ok()
        .header("Content-Type", "text/plain")
        .body("Hello, World!")
        .build()
}

fn handle_api(req: HttpRequest) -> HttpResponse {
    // handle all paths under /api
    HttpResponseBuilder::ok()
        .header("Content-Type", "application/json")
        .body("{ \"api\": true }")
        .build()
}

// Create a router and register routes
let router = Router::new()
    .route("/hello", handle_hello)       // exact match
    .prefix("/api", handle_api);         // prefix match

// Dispatch requests
#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    router.handle_request(req)
}
```

This will dispatch `/hello` to `handle_hello` and any path starting with `/api` to `handle_api`. If no route matches, a 404 Not Found response is returned.

## Components

### HttpServer Trait

The core of the library is the `HttpServer` trait that provides a simple and intuitive API for handling HTTP requests:

```rust
pub trait HttpServer {
    fn route(&mut self, method: Method, path: &str, handler: HandlerFn);
    fn get(&mut self, path: &str, handler: HandlerFn);
    fn post(&mut self, path: &str, handler: HandlerFn);
    fn put(&mut self, path: &str, handler: HandlerFn);
    fn delete(&mut self, path: &str, handler: HandlerFn);
    fn handle_request(&self, request: Request) -> Pin<Box<dyn Future<Output = Response> + Send>>;
}
```

### Request and Response

The library provides convenient abstractions for HTTP requests and responses:

```rust
// Create a request
let request = Request::new(
    Method::POST,
    "/api/data".to_string(),
    headers,
    body
);

// Parse request body as JSON
let data: MyStruct = request.json()?;

// Create responses
let text_response = Response::text("Hello, World!", 200);
let json_response = Response::json(&my_data, 200)?;
```

## Examples

The library comes with several examples in the `examples` directory:

- **HTTP Server**: Example showing how to create a simple HTTP server

  ```
  cargo run --example http_server
  ```

## API Documentation

### Types

- `Method`: HTTP method enum (GET, POST, PUT, DELETE, etc.)
- `Route`: Structure representing a route with path, method, and handler
- `Request`: Structure representing an HTTP request
- `Response`: Structure representing an HTTP response
- `SimpleHttpServer`: A basic implementation of the HttpServer trait

### Request Methods

- `new()`: Create a new request
- `json<T>()`: Parse the request body as JSON
- `get_header()`: Get a specific header value

### Response Methods

- `new()`: Create a new response
- `json<T>()`: Create a JSON response
- `text()`: Create a text response

### Server Methods

- `new()`: Create a new server instance
- `route()`: Register a route with a method, path, and handler
- `get()`, `post()`, `put()`, `delete()`: Convenience methods for registering routes
- `handle_request()`: Process a request and return a response

## Documentation

To view the documentation locally:

```sh
cargo doc --open
```

This will build the documentation and open it in your default web browser.

## Testing

Run the tests with:

```sh
cargo test
```

## License

This project is licensed under either of:

- MIT License
- Apache License, Version 2.0

at your option.
