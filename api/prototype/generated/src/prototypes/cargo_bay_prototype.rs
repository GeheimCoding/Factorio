#[derive(Debug, serde::Deserialize)]
pub struct CargoBayPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: f64,
    graphics_set: Option<crate::types::CargoBayConnectableGraphicsSet>,
    hatch_definitions: Option<Vec<crate::types::CargoHatchDefinition>>,
    inventory_size_bonus: crate::types::ItemStackIndex,
    platform_graphics_set: Option<crate::types::CargoBayConnectableGraphicsSet>,
}
fn default_build_grid_size() -> f64 {
    2.0
}
