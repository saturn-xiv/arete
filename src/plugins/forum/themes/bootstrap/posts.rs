use askama::Template;

#[derive(Template)]
#[template(path = "bootstrap/forum/posts/index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "bootstrap/forum/posts/show.html")]
pub struct Show {}
