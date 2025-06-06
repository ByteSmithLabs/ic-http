use std::collections::HashMap;

use crate::types::Handler;
use ic_http_certification::{HttpRequest, HttpResponse};
use matchit::Router;

pub struct Server {
    fallback: Box<Handler>,
    pub routers: HashMap<String, Router<Box<Handler>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            fallback: Box::new(move |_: &HttpRequest<'static>| {
                Box::pin(async {
                    HttpResponse::not_found(
                        b"Not Found",
                        vec![("Content-Type".into(), "text/plain".into())],
                    )
                    .build()
                })
            }),
            routers: HashMap::new(),
        }
    }

    pub fn is_http_method(method: &str) -> bool {
        matches!(
            method.to_uppercase().as_str(),
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "OPTIONS" | "HEAD" | "TRACE" | "CONNECT"
        )
    }

    pub fn with_fallback(&mut self, handler: Handler) {
        self.fallback = Box::new(handler);
    }

    pub fn route(&mut self, method: &str, path: &str, handler: Handler) {
        if !Self::is_http_method(method) {
            panic!("Invalid HTTP method: {}", method);
        }

        let router = self
            .routers
            .entry(method.to_string())
            .or_insert_with(Router::new);

        router
            .insert(path, Box::new(handler))
            .expect("Failed to insert route into router");
    }

    pub async fn handle(&self, req: &HttpRequest<'static>) -> HttpResponse<'static> {
        let req_path = req.get_path().expect("Failed to get req path");
        let method = req.method().as_str().to_uppercase();

        if let Some(router) = self.routers.get(&method) {
            if let Ok(handler_match) = router.at(&req_path) {
                // Pass a reference to handler_match.params
                return (handler_match.value)(&req).await;
            }
        }

        (self.fallback)(&req).await
    }
}
