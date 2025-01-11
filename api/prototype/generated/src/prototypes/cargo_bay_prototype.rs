pub struct CargoBayPrototype {
    build_grid_size: String,
    graphics_set: crate::types::CargoBayConnectableGraphicsSet,
    hatch_definitions: Vec<crate::types::CargoHatchDefinition>,
    inventory_size_bonus: crate::types::ItemStackIndex,
    platform_graphics_set: crate::types::CargoBayConnectableGraphicsSet,
}
