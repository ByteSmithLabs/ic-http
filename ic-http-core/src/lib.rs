//! # IC HTTP Library
//!
//! `ic-http` is a http library to handle canister request inside canister.
//!
//! ## Features
//!
//! - HTTP server trait for handling HTTP requests

mod server;
mod types;

// Re-export everything to keep the public API clean
pub use server::*;
pub use types::*;
