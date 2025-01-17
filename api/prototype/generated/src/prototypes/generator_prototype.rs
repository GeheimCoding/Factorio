#[derive(serde::Deserialize)]
pub struct GeneratorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    burns_fluid: bool,
    destroy_non_fuel_fluid: bool,
    effectivity: f64,
    energy_source: crate::types::ElectricEnergySource,
    fluid_box: crate::types::FluidBox,
    fluid_usage_per_tick: crate::types::FluidAmount,
    horizontal_animation: crate::types::Animation,
    horizontal_frozen_patch: crate::types::Sprite,
    max_power_output: crate::types::Energy,
    maximum_temperature: f32,
    perceived_performance: crate::types::PerceivedPerformance,
    scale_fluid_usage: bool,
    smoke: Vec<crate::types::SmokeSource>,
    vertical_animation: crate::types::Animation,
    vertical_frozen_patch: crate::types::Sprite,
}
