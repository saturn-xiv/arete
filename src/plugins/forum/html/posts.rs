use super::super::super::super::orm::ID;
use super::super::themes::posts::{Index, Show};

#[get("/posts")]
pub fn index() -> Index {
    Index {}
}

#[get("/posts/<id>")]
pub fn show(id: ID) -> Show {
    debug!("show post {}", id);
    Show {}
}
