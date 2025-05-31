use ic_cdk::{init, post_upgrade, pre_upgrade};
use ic_http::{HttpRequest, HttpResponse, RouteHandler, Server, ServerConfig};
use ic_http_certification::{Method, StatusCode};
use matchit::Router;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static QUERY_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
    static UPDATE_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
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
        .build()
}

fn define_routers() {
    QUERY_ROUTER.with(|router_cell| {
        let mut routers = router_cell.borrow_mut();
        let router = routers
            .entry(Method::GET.to_string())
            .or_insert_with(Router::new);

        router.insert("/ping", handlers::ping).ok();
        router.insert("/test", handlers::test).ok();
        // Add more query routes here as needed
    });

    UPDATE_ROUTER.with(|router_cell| {
        let mut routers = router_cell.borrow_mut();
        let router = routers
            .entry(Method::POST.to_string())
            .or_insert_with(Router::new);
        router.insert("/hello", handlers::hello).ok();
        // Add more update routes here as needed
    });
}

#[ic_cdk::query]
fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();

    ic_cdk::println!("Received request: {:?}", req);
    server.config(ServerConfig {
        query_router: Some(RefCell::new(
            QUERY_ROUTER.with(|cell| cell.borrow().clone()),
        )),
        update_router: None,
    });
    server.query_handle(&req)
}

#[ic_cdk::update]
fn http_request_update(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();
    server.config(ServerConfig {
        query_router: None,
        update_router: Some(RefCell::new(
            UPDATE_ROUTER.with(|cell| cell.borrow().clone()),
        )),
    });
    server.update_handle(&req)
}

#[ic_cdk::update]
fn test(url: String) -> HttpResponse<'static> {
    let mut server = Server::new();
    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url(url)
        .build();

    server.config(ServerConfig {
        query_router: Some(RefCell::new(
            QUERY_ROUTER.with(|cell| cell.borrow().clone()),
        )),
        update_router: Some(RefCell::new(
            UPDATE_ROUTER.with(|cell| cell.borrow().clone()),
        )),
    });

    let res = server.query_handle(&req);

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
