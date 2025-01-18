#[derive(serde::Deserialize)]
pub struct BurnerEnergySource {
    base_: crate::types::BaseEnergySource,
    #[serde(default = "default_burner_usage")]
    burner_usage: crate::types::BurnerUsageID,
    #[serde(default = "default_burnt_inventory_size")]
    burnt_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_effectivity")]
    effectivity: f64,
    // default: `{"chemical"}`
    fuel_categories: Vec<crate::types::FuelCategoryID>,
    fuel_inventory_size: crate::types::ItemStackIndex,
    light_flicker: crate::types::LightFlickeringDefinition,
    smoke: Vec<crate::types::SmokeSource>,
    #[serde(rename = "type")]
    type_: String,
}
fn default_burner_usage() -> crate::types::BurnerUsageID {
    String::from("fuel")
}
fn default_burnt_inventory_size() -> crate::types::ItemStackIndex {
    0
}
fn default_effectivity() -> f64 {
    1.0
}
