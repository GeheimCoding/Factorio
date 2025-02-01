#[derive(Debug, serde::Deserialize)]
pub struct TipsAndTricksItemCategory {
    name: String,
    order: crate::types::Order,
}
