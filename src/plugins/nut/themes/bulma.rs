use askama::Template;

#[derive(Template)]
#[template(path = "bulma/index.html")]
pub struct Index {}
