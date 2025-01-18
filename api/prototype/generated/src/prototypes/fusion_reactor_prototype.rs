#[derive(serde::Deserialize)]
pub struct FusionReactorPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    burner: crate::types::BurnerEnergySource,
    energy_source: crate::types::ElectricEnergySource,
    graphics_set: crate::types::FusionReactorGraphicsSet,
    input_fluid_box: crate::types::FluidBox,
    max_fluid_usage: crate::types::FluidAmount,
    #[serde(default = "default_neighbour_bonus")]
    neighbour_bonus: f32,
    neighbour_connectable: crate::types::NeighbourConnectable,
    output_fluid_box: crate::types::FluidBox,
    perceived_performance: crate::types::PerceivedPerformance,
    power_input: crate::types::Energy,
    #[serde(default = "default_two_direction_only")]
    two_direction_only: bool,
}
fn default_neighbour_bonus() -> f32 {
    1.0
}
fn default_two_direction_only() -> bool {
    false
}
