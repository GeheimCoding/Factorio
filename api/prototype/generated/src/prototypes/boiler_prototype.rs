pub struct BoilerPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    burning_cooldown: u16,
    energy_consumption: crate::types::Energy,
    energy_source: crate::types::EnergySource,
    fire_flicker_enabled: bool,
    fire_glow_flicker_enabled: bool,
    fluid_box: crate::types::FluidBox,
    mode: BoilerPrototypeMode,
    output_fluid_box: crate::types::FluidBox,
    pictures: BoilerPictureSet,
    target_temperature: f32,
}
#[derive(serde::Deserialize)]
pub enum BoilerPrototypeMode {
    #[serde(rename = "heat_fluid_inside")]
    HeatFluidInside,
    #[serde(rename = "output_to_separate_pipe")]
    OutputToSeparatePipe,
}
pub struct BoilerPictureSet {
    east: crate::types::BoilerPictures,
    north: crate::types::BoilerPictures,
    south: crate::types::BoilerPictures,
    west: crate::types::BoilerPictures,
}
