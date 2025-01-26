#[derive(Debug, serde::Deserialize)]
pub struct SpacePlatformHubPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_build_grid_size")]
    build_grid_size: f64,
    cargo_station_parameters: crate::types::CargoStationParameters,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    default_damage_taken_signal: Option<crate::types::SignalIDConnector>,
    default_speed_signal: Option<crate::types::SignalIDConnector>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    dump_container: crate::types::EntityID,
    graphics_set: Option<crate::types::CargoBayConnectableGraphicsSet>,
    inventory_size: crate::types::ItemStackIndex,
    persistent_ambient_sounds: Option<crate::types::PersistentWorldAmbientSoundsDefinition>,
    #[serde(default = "default_platform_repair_speed_modifier")]
    platform_repair_speed_modifier: f32,
    surface_render_parameters: Option<crate::types::SurfaceRenderParameters>,
    #[serde(default = "default_weight")]
    weight: crate::types::Weight,
}
fn default_build_grid_size() -> f64 {
    256.0
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_platform_repair_speed_modifier() -> f32 {
    1.0
}
fn default_weight() -> crate::types::Weight {
    0.0
}
