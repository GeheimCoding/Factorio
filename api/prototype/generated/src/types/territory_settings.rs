#[derive(serde::Deserialize)]
pub struct TerritorySettings {
    #[serde(default = "default_minimum_territory_size")]
    minimum_territory_size: u32,
    territory_index_expression: String,
    // default: 0
    territory_variation_expression: String,
    units: Vec<crate::types::EntityID>,
}
fn default_minimum_territory_size() -> u32 {
    0
}
