use async_trait::async_trait;
use ic_http_certification::{HttpRequest, HttpResponse};
use matchit::{Params, Router};
use std::{cell::RefCell, collections::HashMap};

#[async_trait]
pub trait HandlerTrait {
    async fn handle(&self, req: &HttpRequest, params: &Params) -> HttpResponse<'static>;
    fn clone_box(&self) -> Box<dyn HandlerTrait + Send + Sync>;
}

impl Clone for Box<dyn HandlerTrait + Send + Sync> {
    fn clone(&self) -> Box<dyn HandlerTrait + Send + Sync> {
        Box::new(self.clone_box())
    }
}

pub type Handler = Box<dyn HandlerTrait + Send + Sync>;

/// Server configuration struct
#[derive(Clone)]
pub struct ServerConfig<'a> {
    pub router: &'a RefCell<HashMap<String, Router<Handler>>>,
}
