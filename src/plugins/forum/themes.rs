use super::super::super::{errors::Result, graphql::context::Context};

pub trait Topic {
    fn latest(lang: &String, ctx: &Context, page: i64) -> Result<String>;
    fn by_tag(lang: &String, ctx: &Context, tag: i64) -> Result<String>;
    fn by_category(lang: &String, ctx: &Context, tag: i64) -> Result<String>;
    fn show(lang: &String, ctx: &Context, id: i64) -> Result<String>;
}

pub trait Post {
    fn latest(lang: &String, ctx: &Context, page: i64) -> Result<String>;
}
