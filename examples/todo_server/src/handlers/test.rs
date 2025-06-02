use crate::create_response;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};

pub async fn handle_test(req: &HttpRequest<'static>) -> HttpResponse<'static> {
    create_response(StatusCode::NOT_FOUND, b"123".to_vec())
}
