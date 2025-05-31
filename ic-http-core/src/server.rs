use std::collections::HashMap;

use ic_http_certification::{HttpRequest, HttpResponse};
use std::cell::RefCell;

use crate::RouteHandler;

use matchit::Router;

thread_local! {
    static QUERY_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
    static UPDATE_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
}

/// Simple HTTP server with unified routing
pub struct Server {
    fallback: RouteHandler,
}

impl Server {
    /// Create a new HTTP server with a default fallback handler
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
        }
    }

    /// Set a custom fallback handler
    pub fn with_fallback(mut self, handler: RouteHandler) -> Self {
        self.fallback = handler;
        self
    }

    /// Register a query route
    pub fn query_route(self, method: &str, path: &str, handler: RouteHandler) -> Self {
        QUERY_ROUTER.with(|routers| {
            let mut routers = routers.borrow_mut();
            let router = routers
                .entry(method.to_string())
                .or_insert_with(Router::new);
            router.insert(path, handler).ok();
        });
        self
    }

    /// Register an update route
    pub fn update_route(self, method: &str, path: &str, handler: RouteHandler) -> Self {
        UPDATE_ROUTER.with(|routers| {
            let mut routers = routers.borrow_mut();
            let router = routers
                .entry(method.to_string())
                .or_insert_with(Router::new);
            router.insert(path, handler).ok();
        });
        self
    }

    pub fn query_handle(&self, req: &HttpRequest) -> HttpResponse<'static> {
        let req_path = req.get_path().expect("Failed to get req path");

        QUERY_ROUTER.with_borrow(|query_router| {
            if let Some(method_router) = query_router.get(&req.method().as_str().to_uppercase()) {
                if let Ok(handler_match) = method_router.at(&req_path) {
                    let handler = handler_match.value;
                    return handler(&req, &handler_match.params);
                }
            }
            (self.fallback)(req, &matchit::Params::new())
        })
    }

    pub fn update_handle(&self, req: &HttpRequest) -> HttpResponse<'static> {
        let req_path = req.get_path().expect("Failed to get req path");

        UPDATE_ROUTER.with_borrow(|update_router| {
            if let Some(method_router) = update_router.get(&req.method().as_str().to_uppercase()) {
                if let Ok(handler_match) = method_router.at(&req_path) {
                    let handler = handler_match.value;
                    return handler(&req, &handler_match.params);
                }
            }
            (self.fallback)(req, &matchit::Params::new())
        })
    }
}
