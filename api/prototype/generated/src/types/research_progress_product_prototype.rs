#[derive(serde::Deserialize)]
pub struct ResearchProgressProductPrototype {
    amount: f64,
    research_item: crate::types::ItemID,
    #[serde(rename = "type")]
    type_: String,
}
