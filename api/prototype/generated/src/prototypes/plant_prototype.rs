#[derive(serde::Deserialize)]
pub struct PlantPrototype {
    base_: crate::prototypes::TreePrototype,
    agricultural_tower_tint: crate::types::RecipeTints,
    growth_ticks: crate::types::MapTick,
    harvest_emissions: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
}
