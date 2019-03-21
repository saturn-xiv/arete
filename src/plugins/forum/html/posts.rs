use super::super::themes::posts::{Index, Show};

pub fn index((): ()) -> Index {
    Index {}
}

pub fn show(_id: i64) -> Show {
    Show {}
}
