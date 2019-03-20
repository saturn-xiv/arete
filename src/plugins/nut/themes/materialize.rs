use askama::Template;

#[derive(Template)]
#[template(path = "materialize/index.html")]
pub struct Index {}
