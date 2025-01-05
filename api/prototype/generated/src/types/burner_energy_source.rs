pub struct BurnerEnergySource {
    burner_usage: crate::types::BurnerUsageID,
    burnt_inventory_size: crate::types::ItemStackIndex,
    effectivity: f64,
    fuel_categories: Vec<crate::types::FuelCategoryID>,
    fuel_inventory_size: crate::types::ItemStackIndex,
    light_flicker: crate::types::LightFlickeringDefinition,
    smoke: Vec<crate::types::SmokeSource>,
    type_: String,
}
