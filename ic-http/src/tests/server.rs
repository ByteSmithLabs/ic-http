use ic_http_certification::{HttpRequest, HttpResponse, Method};

use crate::Server;

#[test]
fn test_server_query_route_and_handle() {
    let server = Server::new().query_route(&Method::GET, "/hello", |req, _params| {
        HttpResponse::ok(
            b"Hello, World!",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    });

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/hello")
        .with_headers(vec![("X-Custom-Foo".into(), "Bar".into())])
        .with_body(&[1, 2, 3])
        .with_certificate_version(2)
        .build();

    let res = server.query_handle(&req);
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Hello, World!");
}

#[test]
fn test_server_query_route_not_found() {
    let server = Server::new().query_route(&Method::GET, "/hello", |_, _| {
        HttpResponse::ok(b"Hello, World!", vec![]).build()
    });

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/not-exist")
        .build();

    let res = server.query_handle(&req);
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_query_custom_fallback() {
    use crate::RouteHandler;
    use ic_http_certification::HttpResponse;
    let custom_fallback: RouteHandler = |_, _| HttpResponse::ok(b"Custom Fallback", vec![]).build();
    let server = Server::new()
        .query_route(&Method::GET, "/hello", |_, _| {
            HttpResponse::ok(b"Hello, World!", vec![]).build()
        })
        .with_fallback(custom_fallback);

    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url("/not-exist")
        .build();

    let res = server.query_handle(&req);
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Custom Fallback");
}

#[test]
fn test_server_update_route_and_handle() {
    let server = Server::new().update_route(&Method::POST, "/update", |req, _params| {
        HttpResponse::ok(
            b"Update OK!",
            vec![("Content-Type".into(), "text/plain".into())],
        )
        .build()
    });

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/update")
        .with_body(b"update-body")
        .build();

    let res = server.update_handle(&req);
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Update OK!");
}

#[test]
fn test_server_update_route_not_found() {
    let server = Server::new().update_route(&Method::POST, "/update", |_, _| {
        HttpResponse::ok(b"Update OK!", vec![]).build()
    });

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/not-exist")
        .build();

    let res = server.update_handle(&req);
    assert_eq!(res.status_code(), 404);
    assert_eq!(res.body(), b"Not Found!");
}

#[test]
fn test_server_update_custom_fallback() {
    use crate::RouteHandler;
    use ic_http_certification::HttpResponse;
    let custom_fallback: RouteHandler = |_, _| HttpResponse::ok(b"Update Fallback", vec![]).build();
    let server = Server::new()
        .update_route(&Method::POST, "/update", |_, _| {
            HttpResponse::ok(b"Update OK!", vec![]).build()
        })
        .with_fallback(custom_fallback);

    let req = HttpRequest::builder()
        .with_method(Method::POST)
        .with_url("/not-exist")
        .build();

    let res = server.update_handle(&req);
    assert_eq!(res.status_code(), 200);
    assert_eq!(res.body(), b"Update Fallback");
}
