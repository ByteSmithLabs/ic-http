use crate::create_response;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use serde_json::json;

pub async fn handle_ping(req: &HttpRequest<'static>) -> HttpResponse<'static> {
    let body = serde_json::to_vec(&json!({ "response": "Processed Update" })).unwrap();
    create_response(StatusCode::OK, body)
}
