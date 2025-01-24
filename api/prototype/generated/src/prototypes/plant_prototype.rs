#[derive(serde::Deserialize)]
pub struct PlantPrototype {
    base_: crate::prototypes::TreePrototype,
    agricultural_tower_tint: Option<crate::types::RecipeTints>,
    growth_ticks: crate::types::MapTick,
    harvest_emissions: Option<std::collections::HashMap<crate::types::AirbornePollutantID, f64>>,
}
