pub struct FluidEnergySource {
    burns_fluid: bool,
    destroy_non_fuel_fluid: bool,
    effectivity: f64,
    fluid_box: crate::types::FluidBox,
    fluid_usage_per_tick: crate::types::FluidAmount,
    light_flicker: crate::types::LightFlickeringDefinition,
    maximum_temperature: f32,
    scale_fluid_usage: bool,
    smoke: Vec<crate::types::SmokeSource>,
    type_: String,
}
