use std::cell::RefCell;
use std::collections::HashMap;

use ic_http_certification::{HttpRequest, HttpResponse, Method};

use crate::HandlerTrait;
use crate::{Handler, ServerConfig};
use async_trait::async_trait;
use matchit::Params;
use matchit::Router;

struct DefaultFallbackHandler;

#[async_trait]
impl HandlerTrait for DefaultFallbackHandler {
    async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
        HttpResponse::not_found(
            b"Not Found!",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    }

    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(DefaultFallbackHandler)
    }
}

pub struct Server {
    fallback: Handler,
    pub routers: RefCell<HashMap<String, Router<Handler>>>,
}

#[async_trait]
pub trait AsyncHandler: Send + Sync {
    async fn handle(&self, req: &HttpRequest, params: &Params) -> HttpResponse<'static>;
}

impl Server {
    pub fn new() -> Self {
        Self {
            fallback: Box::new(DefaultFallbackHandler),
            routers: RefCell::new(HashMap::new()),
        }
    }

    pub fn config(&mut self, config_options: ServerConfig) {
        self.routers = config_options.router.clone();
    }

    /// Set a custom fallback handler (must match async signature)
    pub fn with_fallback(&mut self, handler: Handler) {
        self.fallback = handler;
    }

    /// Register a route for any HTTP method
    pub fn route(&mut self, method: &Method, path: &str, handler: Handler) {
        let mut routers = self.routers.borrow_mut(); // Borrow the HashMap mutably
        let router = routers
            .entry(method.to_string())
            .or_insert_with(Router::new);
        router.insert(path, handler).ok();
    }

    pub async fn handle(&self, req: &HttpRequest<'static>) -> HttpResponse<'static> {
        let req_path = req.get_path().expect("Failed to get req path");
        let method = req.method().as_str().to_uppercase();
        let routers = self.routers.borrow_mut(); // Borrow the HashMap mutably

        if let Some(router) = routers.get(&method) {
            if let Ok(handler_match) = router.at(&req_path) {
                return handler_match.value.handle(req, &handler_match.params).await;
            }
        }
        // Fallback handler if no route matched
        self.fallback.handle(req, &Params::new()).await
    }
}
