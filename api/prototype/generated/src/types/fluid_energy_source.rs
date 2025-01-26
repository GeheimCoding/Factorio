#[derive(Debug, serde::Deserialize)]
pub struct FluidEnergySource {
    base_: crate::types::BaseEnergySource,
    #[serde(default = "default_burns_fluid")]
    burns_fluid: bool,
    #[serde(default = "default_destroy_non_fuel_fluid")]
    destroy_non_fuel_fluid: bool,
    #[serde(default = "default_effectivity")]
    effectivity: f64,
    fluid_box: crate::types::FluidBox,
    #[serde(default = "default_fluid_usage_per_tick")]
    fluid_usage_per_tick: crate::types::FluidAmount,
    light_flicker: Option<crate::types::LightFlickeringDefinition>,
    #[serde(default = "default_maximum_temperature")]
    maximum_temperature: f32,
    #[serde(default = "default_scale_fluid_usage")]
    scale_fluid_usage: bool,
    smoke: Option<Vec<crate::types::SmokeSource>>,
    #[serde(rename = "type")]
    type_: String,
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
fn default_fluid_usage_per_tick() -> crate::types::FluidAmount {
    0.0
}
fn default_maximum_temperature() -> f32 {
    0.0
}
fn default_scale_fluid_usage() -> bool {
    false
}
