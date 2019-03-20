use askama::Template;

#[derive(Template)]
#[template(path = "bootstrap/index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "bootstrap/contact.html")]
pub struct Contact {}

#[derive(Template)]
#[template(path = "bootstrap/about.html")]
pub struct About {}
