use askama::Template;

#[derive(Template)]
#[template(path = "bulma/forum/index.html")]
pub struct Index {}
