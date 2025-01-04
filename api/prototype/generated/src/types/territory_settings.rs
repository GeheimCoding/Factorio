pub struct TerritorySettings {
    minimum_territory_size: u32,
    territory_index_expression: String,
    territory_variation_expression: String,
    units: Vec<crate::types::EntityID>,
}
