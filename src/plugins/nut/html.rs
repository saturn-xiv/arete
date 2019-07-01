use super::themes::{About, Contact, Index};

#[get("/")]
pub fn index() -> Index {
    Index {}
}

#[get("/about")]
pub fn about() -> About {
    About {}
}

#[get("/contact")]
pub fn contact() -> Contact {
    Contact {}
}
