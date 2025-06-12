use ic_cdk::{init, post_upgrade, pre_upgrade};
use ic_http::server::Server;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};

mod handlers;

pub(crate) fn create_response(status_code: StatusCode, body: Vec<u8>) -> HttpResponse<'static> {
    HttpResponse::builder()
        .with_headers(vec![
            (
                "cache-control".to_string(),
                "no-store, max-age=0".to_string(),
            ),
            ("pragma".to_string(), "no-cache".to_string()),
        ])
        .with_body(body)
        .with_upgrade(true)
        .build()
}

fn define_server() -> Server {
    let mut server = Server::new();

    server.route("GET", "/ping", |req| {
        Box::pin(handlers::ping::handle_ping(req))
    });
    server.route("POST", "/ping", |req| {
        Box::pin(handlers::hello::handle_hello(req))
    });

    server
}

#[ic_cdk::query]
async fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let server = define_server();

    server.handle(&req).await
}

#[ic_cdk::update]
async fn http_request_update(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let server = define_server();

    server.handle(&req).await
}

#[init]
fn init() {
    define_server();
}

#[pre_upgrade]
fn pre_upgrade() {}

#[post_upgrade]
fn post_upgrade() {
    define_server();
}

ic_cdk::export_candid!();
