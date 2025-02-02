#[derive(Debug, serde::Deserialize)]
pub struct GeneratorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_burns_fluid")]
    burns_fluid: bool,
    #[serde(default = "default_destroy_non_fuel_fluid")]
    destroy_non_fuel_fluid: bool,
    #[serde(default = "default_effectivity")]
    effectivity: f64,
    energy_source: crate::types::ElectricEnergySource,
    fluid_box: crate::types::FluidBox,
    fluid_usage_per_tick: crate::types::FluidAmount,
    horizontal_animation: Option<crate::types::Animation>,
    horizontal_frozen_patch: Option<crate::types::Sprite>,
    max_power_output: Option<crate::types::Energy>,
    maximum_temperature: f32,
    perceived_performance: Option<crate::types::PerceivedPerformance>,
    #[serde(default = "default_scale_fluid_usage")]
    scale_fluid_usage: bool,
    smoke: Option<Vec<crate::types::SmokeSource>>,
    vertical_animation: Option<crate::types::Animation>,
    vertical_frozen_patch: Option<crate::types::Sprite>,
}
fn default_burns_fluid() -> bool {
    false
}
fn default_destroy_non_fuel_fluid() -> bool {
    true
}
fn default_effectivity() -> f64 {
    1.0
}
fn default_scale_fluid_usage() -> bool {
    false
}
