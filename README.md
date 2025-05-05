# Byte-Smith Rust Workspace

This is a Rust workspace containing HTTP-related libraries and utilities.

## Packages

### [ic-http](./ic-http)

A Rust HTTP library with built-in documentation for common operations.

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Building the workspace

```bash
# Build all packages in the workspace
cargo build

# Build a specific package
cargo build -p ic-http
```

### Running examples

```bash
# Run an example from a specific package
cargo run --example basic_operations -p ic-http
cargo run --example http_request -p ic-http
```

### Running tests

```bash
# Run tests for all packages
cargo test

# Run tests for a specific package
cargo test -p ic-http
```

### Generating documentation

```bash
# Generate and open documentation for all packages
cargo doc --open
```

## Workspace Structure

```
byte-smith/
├── Cargo.toml           # Workspace manifest
├── ic-http/             # HTTP library package
│   ├── Cargo.toml       # Package manifest
│   ├── src/             # Source code
│   │   └── lib.rs       # Library root
│   └── examples/        # Example code
│       ├── basic_operations.rs
│       └── http_request.rs
└── README.md            # This file
```

## License

This project is licensed under either:

- MIT License
- Apache License, Version 2.0

at your option.
