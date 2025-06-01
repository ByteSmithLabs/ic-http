use async_trait::async_trait;
use ic_http::HandlerTrait;
use ic_http_certification::{HttpRequest, HttpResponse, StatusCode};
use matchit::Params;

pub struct HelloHandler;

#[async_trait]
impl HandlerTrait for HelloHandler {
    async fn handle(&self, _req: &HttpRequest, _params: &Params) -> HttpResponse<'static> {
        HttpResponse::builder()
            .with_status_code(StatusCode::OK)
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
            .with_body(b"ping".to_vec())
            .with_upgrade(true)
            .build()
    }

    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(HelloHandler)
    }
}
