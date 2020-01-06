#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub name: String,
    pub path: String,
    pub icon: String,
    pub children: Vec<Link>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub name: String,
    pub path: String,
}
