#[derive(Debug, serde::Deserialize)]
pub struct TerritorySettings {
    #[serde(default = "default_minimum_territory_size")]
    minimum_territory_size: u32,
    territory_index_expression: Option<String>,
    // default: 0
    territory_variation_expression: Option<String>,
    units: Option<crate::vec::Vec<crate::types::EntityID>>,
}
fn default_minimum_territory_size() -> u32 {
    0
}
