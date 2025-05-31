use ic_cdk::{init, post_upgrade, pre_upgrade};
use ic_http::{HttpRequest, HttpResponse, RouteHandler, Server, ServerConfig};
use ic_http_certification::Method;
use matchit::Router;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static QUERY_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
    static UPDATE_ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
}

fn define_routers() {
    QUERY_ROUTER.with(|router_cell| {
        let mut routers = router_cell.borrow_mut();
        let router = routers
            .entry(Method::GET.to_string())
            .or_insert_with(Router::new);

        router
            .insert("/ping", |_, _| {
                HttpResponse::ok(b"pong", vec![])
                    .with_upgrade(false)
                    .build()
            })
            .ok();
        // Add more query routes here as needed
    });

    UPDATE_ROUTER.with(|router_cell| {
        let mut routers = router_cell.borrow_mut();
        let router = routers
            .entry(Method::POST.to_string())
            .or_insert_with(Router::new);
        router
            .insert("/hello", |_, _| {
                HttpResponse::ok(b"ping", vec![]).with_upgrade(true).build()
            })
            .ok();
        // Add more update routes here as needed
    });
}

#[ic_cdk::update]
fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();
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

    ic_cdk::println!("--- Query Routers ---");
    for (method, router) in server.query_router.borrow().iter() {
        ic_cdk::println!("Query: {} {:?}", method, router);
    }
    ic_cdk::println!("--- Update Routers ---");
    for (method, router) in server.update_router.borrow().iter() {
        ic_cdk::println!("Update: {} {:?}", method, router);
    }

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
