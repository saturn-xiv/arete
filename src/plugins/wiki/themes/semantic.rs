use askama::Template;

#[derive(Template)]
#[template(path = "semantic/wiki/index.html")]
pub struct Index {}
