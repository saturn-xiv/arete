use askama::Template;

#[derive(Template)]
#[template(path = "semantic/forum/index.html")]
pub struct Index {}
