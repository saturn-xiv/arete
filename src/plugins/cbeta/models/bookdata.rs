#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub len: u8,
    pub summary: String,
    pub chinese: String,
    pub english: String,
}
