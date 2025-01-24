#[derive(serde::Deserialize)]
pub struct BoilerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    burning_cooldown: u16,
    energy_consumption: crate::types::Energy,
    energy_source: crate::types::EnergySource,
    #[serde(default = "default_fire_flicker_enabled")]
    fire_flicker_enabled: bool,
    #[serde(default = "default_fire_glow_flicker_enabled")]
    fire_glow_flicker_enabled: bool,
    fluid_box: crate::types::FluidBox,
    #[serde(default = "default_mode")]
    mode: BoilerPrototypeMode,
    output_fluid_box: crate::types::FluidBox,
    pictures: Option<BoilerPictureSet>,
    target_temperature: Option<f32>,
}
fn default_fire_flicker_enabled() -> bool {
    false
}
fn default_fire_glow_flicker_enabled() -> bool {
    false
}
#[derive(serde::Deserialize)]
pub enum BoilerPrototypeMode {
    #[serde(rename = "heat_fluid_inside")]
    HeatFluidInside,
    #[serde(rename = "output_to_separate_pipe")]
    OutputToSeparatePipe,
}
fn default_mode() -> BoilerPrototypeMode {
    BoilerPrototypeMode::HeatFluidInside
}
#[derive(serde::Deserialize)]
pub struct BoilerPictureSet {
    east: crate::types::BoilerPictures,
    north: crate::types::BoilerPictures,
    south: crate::types::BoilerPictures,
    west: crate::types::BoilerPictures,
}
