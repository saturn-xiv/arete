use std::fmt;
use std::ops::Deref;

use failure::Error;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Method, Request, Response as HyperResponse, StatusCode,
};
use mime::{Mime, APPLICATION_JSON, TEXT_HTML_UTF_8, TEXT_PLAIN_UTF_8, TEXT_XML};
use regex::{Captures, Regex};

use super::{
    crypto::sodium::Encryptor as Sodium, errors::Result, graphql::context::Context,
    settings::Dao as SettingDao, themes::Theme,
};

pub struct Router {
    routes: Vec<(Method, Regex, Box<Route>)>,
}

impl fmt::Display for Router {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "METHOD\tRANK\tURI")?;
        for (m, r, _) in self.routes.iter() {
            write!(fmt, "{}\t{}\n", m, r)?;
        }
        Ok(())
    }
}

impl Router {
    pub fn new() -> Self {
        let routes = Vec::new();
        Self { routes: routes }
    }

    pub fn handle<S>(&self, ctx: &Context, req: &Request<S>) -> Result<Option<Response>> {
        let theme = match self.theme(ctx) {
            Ok(v) => v,
            Err(_) => Theme::default(),
        };
        let (method, path) = (req.method(), req.uri().path());
        for (mt, re, rt) in self.routes.iter() {
            if method == mt {
                if let Some(cap) = re.captures(path) {
                    info!("match {}", re);
                    let rs = rt.handle(&theme, ctx, &cap)?;
                    return Ok(Some(rs));
                }
            }
        }

        Ok(None)
    }

    pub fn get<P: AsRef<str>>(&mut self, p: P, r: Box<Route>) -> Result<()> {
        self.add(Method::GET, p, r)
    }

    pub fn post<P: AsRef<str>>(&mut self, p: P, r: Box<Route>) -> Result<()> {
        self.add(Method::POST, p, r)
    }

    pub fn put<P: AsRef<str>>(&mut self, p: P, r: Box<Route>) -> Result<()> {
        self.add(Method::PUT, p, r)
    }

    pub fn patch<P: AsRef<str>>(&mut self, p: P, r: Box<Route>) -> Result<()> {
        self.add(Method::PATCH, p, r)
    }

    pub fn delete<P: AsRef<str>>(&mut self, p: P, r: Box<Route>) -> Result<()> {
        self.add(Method::DELETE, p, r)
    }

    fn add<P: AsRef<str>>(&mut self, m: Method, p: P, r: Box<Route>) -> Result<()> {
        self.routes.push((m, Regex::new(p.as_ref())?, r));
        Ok(())
    }

    fn theme(&self, ctx: &Context) -> Result<Theme> {
        let db = ctx.db()?;
        let db = db.deref();
        let it = SettingDao::get::<&'static str, Theme, Sodium>(db, &ctx.encryptor, &Theme::KEY)?;
        Ok(it)
    }
}

pub trait Route: Sync + Send {
    fn handle(&self, theme: &Theme, ctx: &Context, cap: &Captures) -> Result<Response>;
}

pub enum Response {
    Html(String),
    Text(String),
    Json(String),
    Xml(String),
    File(Mime, Vec<u8>),
    NotFound,
    InternalServerError(Error),
}

macro_rules! content_type {
    ($x:expr, $y:expr, $z:expr) => {{
        let mut res = HyperResponse::new($y);
        match HeaderValue::from_str(&$x.to_string()) {
            Ok(t) => {
                res.headers_mut().insert(CONTENT_TYPE, t);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
        *res.status_mut() = $z;
        res
    }};
}

impl From<Response> for HyperResponse<Body> {
    fn from(it: Response) -> HyperResponse<Body> {
        match it {
            Response::Html(v) => content_type!(TEXT_HTML_UTF_8, Body::from(v), StatusCode::OK),
            Response::Text(v) => content_type!(TEXT_PLAIN_UTF_8, Body::from(v), StatusCode::OK),
            Response::Json(v) => content_type!(APPLICATION_JSON, Body::from(v), StatusCode::OK),
            Response::Xml(v) => content_type!(TEXT_XML, Body::from(v), StatusCode::OK),
            Response::File(t, v) => content_type!(t, Body::from(v), StatusCode::OK),
            Response::NotFound => content_type!(TEXT_PLAIN_UTF_8, Body::empty(), StatusCode::OK),
            Response::InternalServerError(e) => {
                error!("{}", e);
                content_type!(
                    TEXT_PLAIN_UTF_8,
                    Body::from(e.to_string()),
                    StatusCode::INTERNAL_SERVER_ERROR
                )
            }
        }
    }
}
