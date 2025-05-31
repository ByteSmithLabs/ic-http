use std::collections::HashMap;

use ic_http_certification::Method;

use crate::Handler;

pub struct Router {
    routes: HashMap<(String, Method), Box<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// Register a handler for a specific path and method filter.
    pub fn route<H>(&mut self, path: &str, method: Method, handler: H)
    where
        H: Handler + 'static,
    {
        self.routes
            .insert((path.to_string(), method), Box::new(handler));
    }

    /// Find the Handler for a given path and method.
    pub fn find(&self, path: &str, method: Method) -> Option<&Box<dyn Handler>> {
        self.routes.get(&(path.to_string(), method))
    }
}
