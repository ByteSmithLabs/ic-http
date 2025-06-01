use ic_cdk::{init, post_upgrade, pre_upgrade};
use ic_http::{HttpRequest, HttpResponse, RouteHandler, Server, ServerConfig};
use ic_http_certification::{Method, StatusCode};
use matchit::Router;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static ROUTER: RefCell<HashMap<String, Router<RouteHandler>>> = RefCell::new(HashMap::new());
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
    ROUTER.with(|router_cell| {
        let mut routers = router_cell.borrow_mut();
        // GET routes
        let get_router = routers
            .entry(Method::GET.to_string())
            .or_insert_with(Router::new);
        get_router.insert("/ping", handlers::ping).ok();
        get_router.insert("/test", handlers::test).ok();
        // POST routes
        let post_router = routers
            .entry(Method::POST.to_string())
            .or_insert_with(Router::new);
        post_router.insert("/hello", handlers::hello).ok();
    });
}

#[ic_cdk::query]
fn http_request(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();
    server.config(ServerConfig {
        router: Some(RefCell::new(ROUTER.with(|cell| cell.borrow().clone()))),
    });
    // log request and current routes
    ic_cdk::println!("Handling request: {:?}", req);
    for router in server.router.borrow().values() {
        ic_cdk::println!("Route group: {:?}", router);
    }
    server.query_handle(&req)
}

#[ic_cdk::update]
fn http_request_update(req: HttpRequest<'static>) -> HttpResponse<'static> {
    let mut server = Server::new();
    server.config(ServerConfig {
        router: Some(RefCell::new(ROUTER.with(|cell| cell.borrow().clone()))),
    });

    ic_cdk::println!("Handling update request: {:?}", req);

    // log all route groups
    for router in server.router.borrow().values() {
        ic_cdk::println!("Route group: {:?}", router);
    }

    let req_path = req.get_path().expect("Failed to get req path");
    let method = req.method().as_str().to_uppercase();
    let routers = server.router.borrow();
    let maybe_router = routers.get(&method);

    if let Some(router) = maybe_router {
        ic_cdk::println!("Found router for method: {}", method);
        match router.at(&req_path) {
            Ok(handler_match) => {
                ic_cdk::println!("Matching route for path: {}", req_path);
                let handler = handler_match.value;
                return handler(&req, &handler_match.params);
            }
            Err(_) => {
                ic_cdk::println!("No matching route for path: {}", req_path);
            } // No matching route, fall through to fallback
        }
    } else {
        ic_cdk::println!("No route matched, using fallback handler");
    }

    server.update_handle(&req)
}

#[ic_cdk::update]
fn test(url: String) -> HttpResponse<'static> {
    let mut server = Server::new();
    let req = HttpRequest::builder()
        .with_method(Method::GET)
        .with_url(url)
        .build();

    ic_cdk::println!("Handling request: {:?}", req);

    server.config(ServerConfig {
        router: Some(RefCell::new(ROUTER.with(|cell| cell.borrow().clone()))),
    });

    let res = server.query_handle(&req);

    ic_cdk::println!("Response Status: {}", res.status_code());

    for router in server.router.borrow().values() {
        ic_cdk::println!("Route group: {:?}", router);
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
