use crate::create_response;
use async_trait::async_trait;
use ic_http::HandlerTrait;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use matchit::Params;

pub struct TestHandler;

#[async_trait]
impl HandlerTrait for TestHandler {
    async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
        create_response(StatusCode::OK, b"pong".to_vec())
    }

    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(TestHandler)
    }
}
