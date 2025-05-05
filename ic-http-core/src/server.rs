//! HTTP Server trait and implementation
//!
//! This module provides the `HttpServer` trait that defines a common interface for HTTP servers,
//! along with a simple implementation (`SimpleHttpServer`).
//!
//! The HTTP server framework follows an Express.js or Axum-like API design for defining routes
//! and handling requests, making it intuitive for developers familiar with those frameworks.
//!
//! # Examples
//!
//! ```
//! use ic_http::{SimpleHttpServer, HttpServer, Request, Response, Method};
//! use std::collections::HashMap;
//!
//! // Create a new server
//! let mut server = SimpleHttpServer::new();
//!
//! // Define routes
//! server.get("/hello", |_req| {
//!     Box::pin(async move {
//!         Response::text("Hello, World!", 200)
//!     })
//! });
//!
//! server.post("/echo", |req| {
//!     Box::pin(async move {
//!         // Echo back the request body as a response
//!         Response::new(200, req.headers.clone(), req.body.clone())
//!     })
//! });
//!
//! // Create a request to test the server
//! let request = Request::new(
//!     Method::GET,
//!     "/hello".to_string(),
//!     HashMap::new(),
//!     Vec::new()
//! );
//!
//! // In a real application, you would run this in an async runtime
//! let future_response = server.handle_request(request);
//! ```

use std::future::Future;
use std::pin::Pin;

use crate::types::Route;
use ic_http_certification::{HttpRequest, HttpResponse};
use ic_http_certification::{Method, StatusCode};

/// Handler function type
///
/// Represents an asynchronous HTTP request handler function that takes a `Request` and
/// returns a `Future` that resolves to a `Response`.
pub type HandlerFn<'a> =
    fn(HttpRequest<'a>) -> Pin<Box<dyn Future<Output = HttpResponse> + Send + 'a>>;

/// HttpServer trait for handling HTTP requests
///
/// This trait defines the core functionality of an HTTP server with an Express.js/Axum-like API.
/// It allows registering routes with different HTTP methods and handling incoming requests.
///
/// Implementations of this trait can be used to create custom HTTP server functionality.
pub trait HttpServer {
    /// Register a route with a method, path, and handler
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method (GET, POST, etc.)
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to call when the route matches
    fn route(&mut self, method: Method, path: &str, handler: HandlerFn);

    /// Helper for GET requests
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to call when the route matches
    fn get(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::GET, path, handler);
    }

    /// Helper for POST requests
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to call when the route matches
    fn post(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::POST, path, handler);
    }

    /// Helper for PUT requests
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to call when the route matches
    fn put(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::PUT, path, handler);
    }

    /// Helper for DELETE requests
    ///
    /// # Arguments
    ///
    /// * `path` - The URL path pattern to match
    /// * `handler` - The handler function to call when the route matches
    fn delete(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::DELETE, path, handler);
    }

    /// Handle a request and return a response
    ///
    /// This method processes an incoming HTTP request, finding the appropriate route handler
    /// and executing it to generate a response.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to handle
    ///
    /// # Returns
    ///
    /// A `Future` that resolves to an HTTP `Response`
    async fn handle_request<'a>(
        &self,
        request: HttpRequest<'a>,
    ) -> Pin<Box<dyn Future<Output = HttpResponse> + Send + 'a>>;
}

/// Default implementation of HttpServer
///
/// A simple HTTP server that stores routes in a vector and matches incoming requests
/// against those routes.
///
/// # Examples
///
/// ```
/// use ic_http::{SimpleHttpServer, HttpServer, Response};
///
/// let mut server = SimpleHttpServer::new();
///
/// // Register routes
/// server.get("/api/health", |_req| {
///     Box::pin(async move {
///         Response::text("OK", 200)
///     })
/// });
/// ```
pub struct SimpleHttpServer<'a> {
    /// List of registered routes
    pub routes: Vec<Route<'a>>,
}

impl<'a> SimpleHttpServer<'a> {
    /// Creates a new SimpleHttpServer
    ///
    /// # Returns
    ///
    /// A new empty `SimpleHttpServer` instance
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
}

impl<'a> Default for SimpleHttpServer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> HttpServer for SimpleHttpServer<'a> {
    fn route(&mut self, method: Method, path: &str, handler: HandlerFn<'a>) {
        self.routes.push(Route {
            path: path.to_string(),
            method,
            handler,
        });
    }

    async fn handle_request<'b>(
        &self,
        request: HttpRequest<'b>,
    ) -> Pin<Box<dyn Future<Output = HttpResponse> + Send + 'b>> {
        let url = request.url();
        let method = request.method();
        // Find matching route
        for route in self.routes.clone().into_iter() {
            if route.path == url && route.method == method {
                // Call the handler with the request
                let handler = route.handler;
                return handler(request);
            }
        }

        // Create a 404 Not Found response wrapped in a Future
        Box::pin(async {
            HttpResponse::builder()
                .with_status_code(StatusCode::NOT_FOUND)
                .with_upgrade(true)
                .build()
        })
    }

    fn get(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::GET, path, handler);
    }

    fn post(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::POST, path, handler);
    }

    fn put(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::PUT, path, handler);
    }

    fn delete(&mut self, path: &str, handler: HandlerFn) {
        self.route(Method::DELETE, path, handler);
    }
}
