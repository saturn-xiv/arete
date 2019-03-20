use actix_web::Path;

use super::super::themes::posts::{Index, Show};

pub fn index((): ()) -> Index {
    Index {}
}

pub fn show(_id: Path<i64>) -> Show {
    Show {}
}
