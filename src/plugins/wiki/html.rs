use super::themes::{Index, Show};

#[get("/")]
pub fn index() -> Index {
    Index {}
}

#[get("/<name>")]
pub fn show(name: String) -> Show {
    debug!("show wiki {}", name);
    Show {}
}
