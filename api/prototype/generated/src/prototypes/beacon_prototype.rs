#[derive(serde::Deserialize)]
pub struct BeaconPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    // default: No effects are allowed
    allowed_effects: crate::types::EffectTypeLimitation,
    // default: All module categories are allowed
    allowed_module_categories: Vec<crate::types::ModuleCategoryID>,
    animation: crate::types::Animation,
    base_picture: crate::types::Animation,
    #[serde(default = "default_beacon_counter")]
    beacon_counter: BeaconPrototypeBeaconCounter,
    distribution_effectivity: f64,
    distribution_effectivity_bonus_per_quality_level: f64,
    energy_source: BeaconPrototypeEnergySource,
    energy_usage: crate::types::Energy,
    graphics_set: crate::types::BeaconGraphicsSet,
    module_slots: crate::types::ItemStackIndex,
    perceived_performance: crate::types::PerceivedPerformance,
    profile: Vec<f64>,
    radius_visualisation_picture: crate::types::Sprite,
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
