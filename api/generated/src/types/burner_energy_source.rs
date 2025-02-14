#[derive(Debug, serde::Deserialize)]
pub struct BurnerEnergySource {
    #[serde(flatten)]
    base_: crate::types::BaseEnergySource,
    #[serde(default = "default_burner_usage")]
    burner_usage: crate::types::BurnerUsageID,
    #[serde(default = "default_burnt_inventory_size")]
    burnt_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_effectivity")]
    effectivity: f64,
    // default: `{"chemical"}`
    fuel_categories: Option<crate::vec::Vec<crate::types::FuelCategoryID>>,
    fuel_inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_initial_fuel")]
    initial_fuel: crate::types::ItemID,
    #[serde(default = "default_initial_fuel_percent")]
    initial_fuel_percent: f64,
    light_flicker: Option<crate::types::LightFlickeringDefinition>,
    smoke: Option<crate::vec::Vec<crate::types::SmokeSource>>,
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
fn default_initial_fuel() -> crate::types::ItemID {
    String::from("")
}
fn default_initial_fuel_percent() -> f64 {
    0.2
}
