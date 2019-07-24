#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub class: String,
    pub name: String,
    pub extra: Option<String>,
    pub cid: String,
    pub bid: String,
    pub volume: u16,
    pub book: String,
    pub authors: String,
}
