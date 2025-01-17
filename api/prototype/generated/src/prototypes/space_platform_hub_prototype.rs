#[derive(serde::Deserialize)]
pub struct SpacePlatformHubPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    build_grid_size: String,
    cargo_station_parameters: crate::types::CargoStationParameters,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    default_damage_taken_signal: crate::types::SignalIDConnector,
    default_speed_signal: crate::types::SignalIDConnector,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    dump_container: crate::types::EntityID,
    graphics_set: crate::types::CargoBayConnectableGraphicsSet,
    inventory_size: crate::types::ItemStackIndex,
    persistent_ambient_sounds: crate::types::PersistentWorldAmbientSoundsDefinition,
    platform_repair_speed_modifier: f32,
    surface_render_parameters: crate::types::SurfaceRenderParameters,
    weight: crate::types::Weight,
}
