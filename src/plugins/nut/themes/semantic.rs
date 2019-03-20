use askama::Template;

#[derive(Template)]
#[template(path = "semantic/index.html")]
pub struct Index {}
