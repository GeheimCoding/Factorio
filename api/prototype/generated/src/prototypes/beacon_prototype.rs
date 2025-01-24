#[derive(serde::Deserialize)]
pub struct BeaconPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    // default: No effects are allowed
    allowed_effects: Option<crate::types::EffectTypeLimitation>,
    // default: All module categories are allowed
    allowed_module_categories: Option<Vec<crate::types::ModuleCategoryID>>,
    animation: Option<crate::types::Animation>,
    base_picture: Option<crate::types::Animation>,
    #[serde(default = "default_beacon_counter")]
    beacon_counter: BeaconPrototypeBeaconCounter,
    distribution_effectivity: f64,
    distribution_effectivity_bonus_per_quality_level: Option<f64>,
    energy_source: BeaconPrototypeEnergySource,
    energy_usage: crate::types::Energy,
    graphics_set: Option<crate::types::BeaconGraphicsSet>,
    module_slots: crate::types::ItemStackIndex,
    perceived_performance: Option<crate::types::PerceivedPerformance>,
    profile: Option<Vec<f64>>,
    radius_visualisation_picture: Option<crate::types::Sprite>,
    supply_area_distance: f64,
}
#[derive(serde::Deserialize)]
pub enum BeaconPrototypeBeaconCounter {
    #[serde(rename = "total")]
    Total,
    #[serde(rename = "same_type")]
    SameType,
}
fn default_beacon_counter() -> BeaconPrototypeBeaconCounter {
    BeaconPrototypeBeaconCounter::Total
}
#[derive(serde::Deserialize)]
pub enum BeaconPrototypeEnergySource {
    #[serde(untagged)]
    ElectricEnergySource(Box<crate::types::ElectricEnergySource>),
    #[serde(untagged)]
    VoidEnergySource(Box<crate::types::VoidEnergySource>),
}
