use handlers::ping;
use ic_cdk::{init, post_upgrade, pre_upgrade};
use ic_http::{Handler, Server, ServerConfig};
use ic_http_certification::{HttpRequest, HttpResponse, Method, StatusCode};
use matchit::Router;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static ROUTER: RefCell<HashMap<String, Router<Handler>>> = RefCell::new(HashMap::new());
}

// register handler modules
mod handlers;

pub(crate) fn create_response(status_code: StatusCode, body: Vec<u8>) -> HttpResponse<'static> {
    HttpResponse::builder()
        .with_status_code(status_code)
        .with_headers(vec![
            ("content-type".to_string(), "application/json".to_string()),
            (
                "strict-transport-security".to_string(),
                "max-age=31536000; includeSubDomains".to_string(),
            ),
            ("x-content-type-options".to_string(), "nosniff".to_string()),
            ("referrer-policy".to_string(), "no-referrer".to_string()),
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

fn define_routers() {
    ROUTER.with_borrow_mut(|routers| {
        // GET routes
        routers
            .entry(Method::GET.to_string())
            .or_insert_with(Router::new)
            .insert("/ping", Box::new(ping::PingHandler))
            .ok();
        routers
            .get_mut(&Method::GET.to_string())
            .unwrap()
            .insert("/test", Box::new(handlers::test::TestHandler))
            .ok();
        // POST routes
        routers
            .entry(Method::POST.to_string())
            .or_insert_with(Router::new)
            .insert("/hello", Box::new(handlers::hello::HelloHandler))
            .ok();
    });
}

#[ic_cdk::query]
async fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();
    ROUTER.with(|router| {
        server.config(ServerConfig { router: router });
    });
    server.handle(&req).await
}

#[ic_cdk::update]
async fn http_request_update(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();

    ROUTER.with(|router| {
        server.config(ServerConfig { router: router });
    });
    server.handle(&req).await
}

#[ic_cdk::update]
async fn test(url: String) -> HttpResponse<'static> {
    let mut server = Server::new();
    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url(url)
        .build();

    ROUTER.with(|router| {
        server.config(ServerConfig { router: router });
    });

    let res = server.handle(&req).await;

    res
}

#[init]
fn init() {
    define_routers();
}

#[pre_upgrade]
fn pre_upgrade() {}

#[post_upgrade]
fn post_upgrade() {
    define_routers();
}

ic_cdk::export_candid!();
