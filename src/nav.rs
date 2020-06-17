use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub name: String,
    pub path: String,
    pub icon: Option<String>,
    pub children: Vec<Link>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub name: String,
    pub path: String,
    pub params: Option<Map<String, Value>>,
}
