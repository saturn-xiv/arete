use askama::Template;

#[derive(Template)]
#[template(path = "bootstrap/wiki/index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "bootstrap/wiki/show.html")]
pub struct Show {}
