pub struct FluidEnergySource {
    burns_fluid: bool,
    destroy_non_fuel_fluid: bool,
    effectivity: f64,
    fluid_box: FluidBox,
    fluid_usage_per_tick: FluidAmount,
    light_flicker: LightFlickeringDefinition,
    maximum_temperature: f32,
    scale_fluid_usage: bool,
    smoke: Vec<SmokeSource>,
    type_: Fluid,
}
