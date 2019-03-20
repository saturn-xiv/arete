use actix_web::Path;

use super::themes::{Index, Show};

pub fn index((): ()) -> Index {
    Index {}
}

pub fn show(_name: Path<String>) -> Show {
    Show {}
}
