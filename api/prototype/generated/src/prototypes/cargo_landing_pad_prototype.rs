#[derive(serde::Deserialize)]
pub struct CargoLandingPadPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    cargo_station_parameters: crate::types::CargoStationParameters,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    graphics_set: crate::types::CargoBayConnectableGraphicsSet,
    inventory_size: crate::types::ItemStackIndex,
    #[serde(default = "default_radar_range")]
    radar_range: u32,
    robot_animation: crate::types::Animation,
    robot_animation_sound: crate::types::Sound,
    robot_landing_location_offset: crate::types::Vector,
    #[serde(default = "default_robot_opened_duration")]
    robot_opened_duration: u8,
    #[serde(default = "default_trash_inventory_size")]
    trash_inventory_size: crate::types::ItemStackIndex,
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
fn default_radar_range() -> u32 {
    0
}
fn default_robot_opened_duration() -> u8 {
    0
}
fn default_trash_inventory_size() -> crate::types::ItemStackIndex {
    0
}
