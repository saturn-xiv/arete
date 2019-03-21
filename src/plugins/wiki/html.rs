use super::themes::{Index, Show};

pub fn index((): ()) -> Index {
    Index {}
}

pub fn show(_name: String) -> Show {
    Show {}
}
