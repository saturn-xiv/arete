use super::super::themes::topics::{Index, Show};

pub fn index((): ()) -> Index {
    Index {}
}

pub fn show(_id: i64) -> Show {
    Show {}
}
