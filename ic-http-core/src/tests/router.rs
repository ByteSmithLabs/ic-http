// Unit tests for Router
use ic_http_certification::{HttpRequest, HttpResponse, Method};
use std::future::Future;
use std::pin::Pin;

use crate::{Handler, Router};

struct TestHandler;

impl Handler for TestHandler {
    fn call(&self, _req: &HttpRequest<'_>) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> {
        let fut = async move {
            // Use HttpResponse::new as in the server.rs test
            HttpResponse::ok(
                b"Hello, World!",
                vec![("Content-Type".to_string(), "text/plain".to_string())],
            )
            .build()
        };
        Box::pin(fut)
    }
}

#[test]
fn test_router_route_and_find() {
    let mut router = Router::new();
    router.route("/test", Method::GET, TestHandler);
    assert!(router.find("/test", Method::GET).is_some());
    assert!(router.find("/notfound", Method::GET).is_none());
    assert!(router.find("/test", Method::POST).is_none());
}
