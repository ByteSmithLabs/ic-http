use std::collections::HashMap;

use ic_http_certification::{HttpRequest, HttpResponse, Method};
use std::cell::RefCell;

use crate::{RouteHandler, ServerConfig};

use matchit::Router;

#[derive(Clone)]
pub struct Server {
    fallback: RouteHandler,
    pub router: RefCell<HashMap<String, Router<RouteHandler>>>,
}

impl Server {
    pub fn new() -> Self {
        fn default_fallback(
            _req: &HttpRequest,
            _params: &matchit::Params,
        ) -> HttpResponse<'static> {
            HttpResponse::not_found(
                b"Not Found!",
                vec![("Content-Type".into(), "text/plain".into())],
            )
            .build()
        }
        Self {
            fallback: default_fallback,
            router: RefCell::new(HashMap::new()),
        }
    }

    pub fn config(&mut self, config_options: ServerConfig) {
        if let Some(router) = config_options.router {
            self.router = router;
        }
    }

    /// Set a custom fallback handler
    pub fn with_fallback(&mut self, handler: RouteHandler) -> () {
        self.fallback = handler;
    }

    /// Register a query route
    pub fn query_route(&self, method: &Method, path: &str, handler: RouteHandler) -> () {
        let mut routers = self.router.borrow_mut();
        let router = routers
            .entry(method.to_string())
            .or_insert_with(Router::new);
        router.insert(path, handler).ok();
    }

    /// Register an update route
    pub fn update_route(&self, method: &Method, path: &str, handler: RouteHandler) -> () {
        let mut routers = self.router.borrow_mut();
        let router = routers
            .entry(method.to_string())
            .or_insert_with(Router::new);
        router.insert(path, handler).ok();
    }

    pub fn query_handle(&self, req: &HttpRequest) -> HttpResponse<'static> {
        let req_path = req.get_path().expect("Failed to get req path");
        let method = req.method().as_str().to_uppercase();

        let routers = self.router.borrow();
        let maybe_router = routers.get(&method);

        if let Some(router) = maybe_router {
            match router.at(&req_path) {
                Ok(handler_match) => {
                    let handler = handler_match.value;
                    return handler(req, &handler_match.params);
                }
                Err(_) => {} // No matching route, fall through to fallback
            }
        }

        // Fallback handler if no route matched
        (self.fallback)(req, &matchit::Params::new())
    }

    pub fn update_handle(&self, req: &HttpRequest) -> HttpResponse<'static> {
        let req_path = req.get_path().expect("Failed to get req path");
        let method = req.method().as_str().to_uppercase();

        let routers = self.router.borrow();
        let maybe_router = routers.get(&method);

        if let Some(router) = maybe_router {
            match router.at(&req_path) {
                Ok(handler_match) => {
                    let handler = handler_match.value;
                    return handler(req, &handler_match.params);
                }
                Err(_) => {} // No matching route, fall through to fallback
            }
        }

        // Fallback handler if no route matched
        (self.fallback)(req, &matchit::Params::new())
    }
}
