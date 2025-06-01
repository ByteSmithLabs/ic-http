use async_trait::async_trait;
use futures::executor::block_on;
use ic_http_certification::Method;
use ic_http_certification::{HttpRequest, HttpResponse};
use matchit::Params;

use crate::{Handler, HandlerTrait, Server};

/// Helper to initialize a server with default routes
fn setup_server() -> Server {
    let mut server = Server::new();

    struct HelloHandler;
    struct UpdateHandler;

    #[async_trait]
    impl HandlerTrait for HelloHandler {
        async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
            HttpResponse::ok(
                b"Hello, World!",
                vec![("Content-Type".into(), "text/plain".into())],
            )
            .build()
        }

        fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
            Box::new(HelloHandler)
        }
    }

    #[async_trait]
    impl HandlerTrait for UpdateHandler {
        async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
            HttpResponse::ok(
                b"Update OK!",
                vec![("Content-Type".into(), "text/plain".into())],
            )
            .build()
        }

        fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
            Box::new(UpdateHandler)
        }
    }

    server.route(&Method::GET, "/hello", Box::new(HelloHandler));
    server.route(&Method::POST, "/update", Box::new(UpdateHandler));

    server
}

/// Example asynchronous handler
struct AsyncHelloHandler;

#[async_trait]
impl HandlerTrait for AsyncHelloHandler {
    async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
        HttpResponse::ok(
            b"Hello Async!",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    }

    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(AsyncHelloHandler)
    }
}

struct CustomCallbackHandler;

#[async_trait]
impl HandlerTrait for CustomCallbackHandler {
    async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
        HttpResponse::not_found(
            b"Not Found!",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    }

    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(CustomCallbackHandler)
    }
}

#[test]
fn test_server_get_route_and_handle() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/hello")
        .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
        .with_body(&[1, 2, 3])
        .with_certificate_version(2)
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Hello, World!");
}

#[test]
fn test_server_get_route_not_found() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/not-exist")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_get_custom_fallback() {
    let custom_fallback: Handler = Box::new(CustomCallbackHandler);
    let mut server = setup_server();
    server.with_fallback(custom_fallback);

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/not-exist")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_post_route_and_handle() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/update")
        .with_body(b"update-body")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Update OK!");
}

#[test]
fn test_server_post_route_not_found() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/not-exist")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_post_custom_fallback() {
    let custom_fallback = Box::new(CustomCallbackHandler);
    let mut server = setup_server();
    server.with_fallback(custom_fallback);

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/not-exist")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_async_route_and_handle() {
    let mut server = setup_server();
    // register async route
    server.route(&Method::GET, "/async", Box::new(AsyncHelloHandler));

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/async")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Hello Async!");
}

#[test]
fn test_server_async_route_not_found() {
    let mut server = setup_server();
    // register async route
    server.route(&Method::GET, "/async", Box::new(AsyncHelloHandler));

    // request a non-existent path
    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/no-async")
        .build();

    let res = block_on(server.handle(&req));
    // default fallback should return 404 Not Found
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_default_fallback() {
    let server = Server::new();

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/non-existent")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_custom_fallback() {
    let custom_fallback = Box::new(CustomCallbackHandler);

    let mut server = Server::new();
    server.with_fallback(custom_fallback);

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/non-existent")
        .build();

    let res = block_on(server.handle(&req));
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}
