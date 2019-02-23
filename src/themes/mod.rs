pub mod bootstrap;
pub mod materialize;

use super::plugins::{forum, nut, wiki};

pub trait Theme:
    nut::themes::Site + forum::themes::Topic + forum::themes::Post + wiki::themes::Wiki
{
}
