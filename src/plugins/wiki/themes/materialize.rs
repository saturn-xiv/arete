use askama::Template;

#[derive(Template)]
#[template(path = "materialize/wiki/index.html")]
pub struct Index {}
