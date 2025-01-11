pub struct FusionReactorPrototype {
    burner: crate::types::BurnerEnergySource,
    energy_source: crate::types::ElectricEnergySource,
    graphics_set: crate::types::FusionReactorGraphicsSet,
    input_fluid_box: crate::types::FluidBox,
    max_fluid_usage: crate::types::FluidAmount,
    neighbour_bonus: f32,
    neighbour_connectable: crate::types::NeighbourConnectable,
    output_fluid_box: crate::types::FluidBox,
    perceived_performance: crate::types::PerceivedPerformance,
    power_input: crate::types::Energy,
    two_direction_only: bool,
}
