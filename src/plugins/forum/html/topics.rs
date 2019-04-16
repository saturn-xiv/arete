use super::super::super::super::orm::ID;
use super::super::themes::topics::{Index, Show};

#[get("/topics")]
pub fn index() -> Index {
    Index {}
}

#[get("/topics/<id>")]
pub fn show(id: ID) -> Show {
    debug!("show topic {}", id);
    Show {}
}
