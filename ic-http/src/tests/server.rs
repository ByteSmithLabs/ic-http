use std::{future::Future, pin::Pin};

use crate::Server;
use ic_http_certification::{HttpRequest, HttpResponse, Method};

/// Helper to initialize a server with default routes
fn setup_server() -> Server {
    let mut server = Server::new();

    server.route("GET", "/hello", |req| Box::pin(hello_handler(req)));
    server.route("POST", "/update", update_handler);
    server.with_fallback(custom_fallback_handler);

    server
}

/// Handler for the "/hello" route
async fn hello_handler<'a>(req: &'a HttpRequest<'a>) -> HttpResponse<'static> {
    // Simulate some processing for the hello request
    let query_params = req.get_query().unwrap_or_default();
    let name = query_params.unwrap_or("World".to_string());

    // Create a response body
    let body = format!("Hello, {}!", name).into_bytes(); // Convert to owned data (Vec<u8>)

    // Build and return the HTTP response
    HttpResponse::ok(
        body,                                               // Pass owned data
        vec![("Content-Type".into(), "text/plain".into())], // Ensure headers are owned
    )
    .build()
}

/// Handler for the "/update" route
fn update_handler<'a>(
    req: &'a HttpRequest<'a>,
) -> Pin<Box<dyn Future<Output = HttpResponse<'static>> + 'a>> {
    Box::pin(async move {
        // Simulate an async operation
        let simulated_async_result = "Processed Update";

        println!("Async operation result: {}", simulated_async_result);

        HttpResponse::ok(
            simulated_async_result.as_bytes(),
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    })
}

/// Custom fallback handler for unmatched routes
fn custom_fallback_handler<'a>(
    req: &'a HttpRequest<'a>,
) -> Pin<Box<dyn Future<Output = HttpResponse<'static>> + 'a>> {
    Box::pin(async move {
        HttpResponse::not_found(
            b"Custom Not Found",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    })
}

#[tokio::test]
async fn test_server_get_route_and_handle() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/hello")
        .build();

    let res = server.handle(&req).await;
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Hello, World!");
}

#[tokio::test]
async fn test_server_post_route_and_handle() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/update")
        .build();

    let res = server.handle(&req).await;
    println!("Response: {:?}", res);
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Processed Update");
}

#[tokio::test]
async fn test_server_route_not_found() {
    let server = setup_server();

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/not-exist")
        .build();

    let res = server.handle(&req).await;
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Custom Not Found");
}

#[tokio::test]
async fn test_server_custom_fallback() {
    let mut server = setup_server();

    server.with_fallback(custom_fallback_handler);

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/not-exist")
        .build();

    let res = server.handle(&req).await;
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Custom Not Found");
}
