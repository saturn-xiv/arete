use askama::Template;

#[derive(Template)]
#[template(path = "bulma/wiki/index.html")]
pub struct Index {}
