pub mod bootstrap;
pub mod materialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    Bootstrap,
    Materialize,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Bootstrap
    }
}

impl Theme {
    pub const KEY: &'static str = "site.theme";
}
