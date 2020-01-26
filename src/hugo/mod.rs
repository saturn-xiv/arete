pub mod bootstrap;
pub mod bulma;
pub mod materialiaze;
pub mod semantic_ui;

use std::fmt::Debug;

use serde::Serialize;

pub trait Template: Serialize + Debug + Clone {}
