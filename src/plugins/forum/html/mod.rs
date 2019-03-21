pub mod posts;
pub mod topics;

use super::themes::Index;

#[get("/")]
pub fn index() -> Index {
    Index {}
}
