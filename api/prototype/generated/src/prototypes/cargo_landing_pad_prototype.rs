#[derive(serde::Deserialize)]
pub struct CargoLandingPadPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    cargo_station_parameters: crate::types::CargoStationParameters,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    graphics_set: crate::types::CargoBayConnectableGraphicsSet,
    inventory_size: crate::types::ItemStackIndex,
    radar_range: u32,
    robot_animation: crate::types::Animation,
    robot_animation_sound: crate::types::Sound,
    robot_landing_location_offset: crate::types::Vector,
    robot_opened_duration: u8,
    trash_inventory_size: crate::types::ItemStackIndex,
}
