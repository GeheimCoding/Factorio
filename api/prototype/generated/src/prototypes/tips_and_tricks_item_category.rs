#[derive(serde::Deserialize)]
pub struct TipsAndTricksItemCategory {
    name: String,
    order: crate::types::Order,
    #[serde(rename = "type")]
    type_: String,
}
