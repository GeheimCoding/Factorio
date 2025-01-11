pub struct BoilerPrototype {
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
pub enum BoilerPrototypeMode {
    HeatFluidInside,
    OutputToSeparatePipe,
}
pub struct BoilerPictureSet {
    east: crate::types::BoilerPictures,
    north: crate::types::BoilerPictures,
    south: crate::types::BoilerPictures,
    west: crate::types::BoilerPictures,
}
