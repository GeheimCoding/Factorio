#[derive(Debug, serde::Deserialize)]
pub struct ThrusterPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    fuel_fluid_box: crate::types::FluidBox,
    graphics_set: Option<crate::types::ThrusterGraphicsSet>,
    max_performance: crate::types::ThrusterPerformancePoint,
    min_performance: crate::types::ThrusterPerformancePoint,
    oxidizer_fluid_box: crate::types::FluidBox,
    plumes: Option<crate::types::PlumesSpecification>,
}
