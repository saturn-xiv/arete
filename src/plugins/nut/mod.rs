pub mod controllers;
pub mod models;

use std::sync::Arc;

use super::super::env::Config;

pub struct State {
    pub config: Arc<Config>,
}
