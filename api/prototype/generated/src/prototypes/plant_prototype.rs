pub struct PlantPrototype {
    agricultural_tower_tint: crate::types::RecipeTints,
    growth_ticks: crate::types::MapTick,
    harvest_emissions: std::collections::HashMap<crate::types::AirbornePollutantID, f64>,
}
