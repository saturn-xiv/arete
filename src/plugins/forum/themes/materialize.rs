use askama::Template;

#[derive(Template)]
#[template(path = "materialize/forum/index.html")]
pub struct Index {}
