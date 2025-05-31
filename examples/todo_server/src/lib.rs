use ic_http::{HttpRequest, HttpResponse, Server};
use ic_http_certification::Method;

#[ic_cdk::update]
fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let server = Server::new().query_route(&Method::GET, "/ping", |_, _| {
        HttpResponse::ok(b"pong", vec![]).with_upgrade(true).build()
    });
    server.query_handle(&req)
}

#[ic_cdk::update]
fn http_request_update(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let server = Server::new().update_route(&Method::POST, "/hello", |_, _| {
        HttpResponse::ok(b"ping", vec![]).with_upgrade(true).build()
    });
    server.update_handle(&req)
}

ic_cdk::export_candid!();
