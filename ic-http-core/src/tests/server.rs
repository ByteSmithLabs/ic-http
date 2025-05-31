use std::{future::Future, pin::Pin};

// Unit tests for Server
use ic_http_certification::{HttpRequest, HttpResponse, Method};

use crate::{Handler, Server};

struct TestHandler;

impl Handler for TestHandler {
    fn call(
        &self,
        _req: &HttpRequest<'_>,
    ) -> Pin<Box<dyn Future<Output = HttpResponse<'_>> + Send + 'static>> {
        let fut = async move {
            HttpResponse::ok(
                b"Hello, World!",
                vec![("Content-Type".into(), "text/plain".into())],
            )
            .build()
        };
        Box::pin(fut)
    }
}

#[tokio::test]
async fn test_server_route_and_handle() {
    let server = Server::new().route("/hello", Method::GET, TestHandler);
    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/hello")
        .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
        .with_body(&[1, 2, 3])
        .with_certificate_version(2)
        .build();
    let res = server.handle(req).await;
    assert!(res.is_ok());
    let resp = res.unwrap();
    assert_eq!(resp.status_code(), 200);
    assert_eq!(resp.body(), b"Hello, World!");
}
