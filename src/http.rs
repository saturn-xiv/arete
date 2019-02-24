use std::ops::Deref;

use hyper::{Method, Request};
use mime::Mime;
use regex::{Captures, Regex};

use super::{
    crypto::sodium::Encryptor as Sodium, errors::Result, graphql::context::Context,
    settings::Dao as SettingDao, themes::Theme,
};

pub struct Router {
    routes: Vec<(Method, Regex, Box<Route>)>,
}

impl Router {
    pub fn new() -> Result<Self> {
        let routes = Vec::new();
        Ok(Self { routes: routes })
    }

    pub fn handle<S>(&self, ctx: &Context, req: &Request<S>) -> Result<Option<(Mime, Vec<u8>)>> {
        let theme = match self.theme(ctx) {
            Ok(v) => v,
            Err(_) => Theme::default(),
        };
        let (method, path) = (req.method(), req.uri().path());
        for (mt, re, rt) in self.routes.iter() {
            if method == mt {
                if let Some(cap) = re.captures(path) {
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
    fn handle(&self, theme: &Theme, ctx: &Context, cap: &Captures) -> Result<(Mime, Vec<u8>)>;
}
