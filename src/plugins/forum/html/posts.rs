use super::super::themes::posts::{Index, Show};

#[get("/posts")]
pub fn index() -> Index {
    Index {}
}

#[get("/posts/<id>")]
pub fn show(id: i64) -> Show {
    debug!("show post {}", id);
    Show {}
}
