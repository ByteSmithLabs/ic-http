pub use ic_http_certification::{HttpRequest, HttpResponse, HttpResponseBuilder};
use matchit::{Params, Router};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub type RouteHandler = for<'a> fn(&'a HttpRequest, &'a Params) -> HttpResponse<'static>;

pub type RouteHandlerAsync =
    for<'a> fn(
        &'a HttpRequest,
        &'a Params,
    ) -> Pin<Box<dyn Future<Output = HttpResponse<'static>> + Send + 'a>>;

pub struct ServerConfig {
    pub query_router: Option<RefCell<HashMap<String, Router<RouteHandler>>>>,
    pub update_router: Option<RefCell<HashMap<String, Router<RouteHandler>>>>,
}
