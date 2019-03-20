pub mod posts;
pub mod topics;

use askama::Template;

#[derive(Template)]
#[template(path = "bootstrap/forum/index.html")]
pub struct Index {}
