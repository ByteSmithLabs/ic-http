use crate::create_response;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use serde_json::json;
use std::{future::Future, pin::Pin};

pub async fn handle_hello(req: &HttpRequest<'static>) -> HttpResponse<'static> {
    let query_params = req.get_query().unwrap_or_default();
    let name = query_params.unwrap_or("World".to_string());

    let body = format!("Hello, {}!", name).into_bytes(); // Convert to Vec<u8>
    create_response(StatusCode::OK, body)
}
