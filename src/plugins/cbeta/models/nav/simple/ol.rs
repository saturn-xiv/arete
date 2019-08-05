use super::super::super::Text;
use super::Link;

#[derive(Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "span")]
    pub span: Text,
    pub ol: Ol,
}

#[derive(Deserialize, Debug)]
pub struct Ol {
    #[serde(rename = "li")]
    pub items: Vec<Li>,
}

#[derive(Deserialize, Debug)]
pub struct Li {
    #[serde(rename = "$value")]
    pub item: Link,
}
