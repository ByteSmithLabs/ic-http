use std::{future::Future, pin::Pin};

use ic_http_certification::{HttpRequest, HttpResponse};

pub type Handler = for<'a> fn(
    &'a HttpRequest<'static>,
) -> Pin<Box<dyn Future<Output = HttpResponse<'static>> + 'a>>;
