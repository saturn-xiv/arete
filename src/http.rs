use hyper::{Method, Request};
use mime::Mime;

use super::{errors::Result, graphql::context::Context};

pub struct Router {
    pub theme: String,
}

impl Router {
    pub fn handle<S>(&self, ctx: &Context, req: &Request<S>) -> Result<Option<(Mime, String)>> {
        if req.method() != Method::GET {
            return Ok(None);
        }
        Ok(None)
    }
}

pub trait Route {
    type Params;
    fn match_<S>(&self, req: &Request<S>) -> Option<Self::Params>;
    fn handle<S>(&self, theme: &String, ctx: &Context, req: &Request<S>) -> Result<(Mime, String)>;
}
