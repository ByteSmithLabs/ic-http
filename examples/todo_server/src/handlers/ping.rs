use crate::create_response;
use async_trait::async_trait;
use ic_http::HandlerTrait;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use matchit::Params;
use serde_json::json;

pub struct PingHandler;

#[async_trait]
impl HandlerTrait for PingHandler {
    async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
        let body = serde_json::to_vec(&json!({ "response": "pong" })).unwrap();
        create_response(StatusCode::OK, body)
    }

    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(PingHandler)
    }
}
