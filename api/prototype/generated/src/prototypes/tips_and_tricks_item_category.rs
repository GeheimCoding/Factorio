#[derive(serde::Deserialize)]
pub struct TipsAndTricksItemCategory {
    name: String,
    order: crate::types::Order,
    type_: String,
}
