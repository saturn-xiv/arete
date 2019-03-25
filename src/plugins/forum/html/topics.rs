use super::super::themes::topics::{Index, Show};

#[get("/topics")]
pub fn index() -> Index {
    Index {}
}

#[get("/topics/<id>")]
pub fn show(id: i64) -> Show {
    debug!("show topic {}", id);
    Show {}
}
