use super::super::super::{errors::Result, graphql::context::Context};

pub trait Site {
    fn home(lang: &String, ctx: &Context) -> Result<String>;
    fn about(lang: &String, ctx: &Context) -> Result<String>;
}
