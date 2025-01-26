#[derive(Debug, serde::Deserialize)]
pub struct ResearchProgressProductPrototype {
    #[serde(default = "default_amount")]
    amount: f64,
    research_item: crate::types::ItemID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_amount() -> f64 {
    1.0
}
