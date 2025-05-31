use crate::create_response;
use ic_http::{HttpRequest, HttpResponse};
use ic_http_certification::StatusCode;
use matchit::Params;

pub fn hello(_req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
    create_response(StatusCode::OK, b"ping".to_vec())
}
