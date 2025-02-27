#[derive(Debug, serde::Deserialize)]
pub struct FusionGeneratorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    energy_source: crate::types::ElectricEnergySource,
    graphics_set: Option<crate::types::FusionGeneratorGraphicsSet>,
    input_fluid_box: crate::types::FluidBox,
    max_fluid_usage: crate::types::FluidAmount,
    output_fluid_box: crate::types::FluidBox,
    perceived_performance: Option<crate::types::PerceivedPerformance>,
}
