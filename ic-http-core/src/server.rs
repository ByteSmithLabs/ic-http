use std::collections::HashMap;

use ic_http_certification::{HttpRequest, HttpResponse, Method};

use crate::Handler;
/// Simple HTTP server with unified routing
pub struct Server {
    routes: HashMap<(String, Method), Box<dyn Handler>>,
}

impl Server {
    /// Create a new HTTP server
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register an exact-match route and method
    pub fn route<H>(mut self, path: &str, method: Method, handler: H) -> Self
    where
        H: Handler + 'static,
    {
        self.routes
            .insert((path.to_string(), method), Box::new(handler));
        self
    }

    /// Handle an incoming request
    pub async fn handle(&self, req: HttpRequest<'_>) -> Result<HttpResponse, String> {
        let method = req.method();
        let path = req.url();
        if let Some(handler) = self.routes.get(&(path.to_string(), method.clone())) {
            let res = handler.call(&req).await;
            Ok(res)
        } else {
            Err(format!("No handler found for {} {}", method, path))
        }
    }
}
