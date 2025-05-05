use ic_http_certification::Method;

use crate::HandlerFn;

/// Route definition
///
/// Represents a route in the HTTP server with a path, HTTP method, and handler function.
///
/// # Fields
///
/// * `path` - The URL path for this route (e.g., "/api/users")
/// * `method` - The HTTP method for this route
/// * `handler` - The function that will be called to handle requests to this route
#[derive(Debug, Clone)]
pub struct Route<'a> {
    pub path: String,
    pub method: Method,
    pub handler: HandlerFn<'a>,
}
