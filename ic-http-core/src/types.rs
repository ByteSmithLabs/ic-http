pub use ic_http_certification::{HttpRequest, HttpResponse, HttpResponseBuilder};
use std::future::Future;
use std::pin::Pin;
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'static;
    fn call(&mut self, req: Request) -> Self::Future;
}

pub trait Handler: Send + Sync {
    fn call(&self, req: &HttpRequest) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>>;
}
