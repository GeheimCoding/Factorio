#[derive(Debug, serde::Deserialize)]
pub struct RadarPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_connects_to_other_radars")]
    connects_to_other_radars: bool,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_energy_fraction_to_connect")]
    energy_fraction_to_connect: f32,
    #[serde(default = "default_energy_fraction_to_disconnect")]
    energy_fraction_to_disconnect: f32,
    energy_per_nearby_scan: crate::types::Energy,
    energy_per_sector: crate::types::Energy,
    energy_source: crate::types::EnergySource,
    energy_usage: crate::types::Energy,
    frozen_patch: Option<crate::types::Sprite>,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    max_distance_of_nearby_sector_revealed: u32,
    max_distance_of_sector_revealed: u32,
    pictures: Option<crate::types::RotatedSprite>,
    radius_minimap_visualisation_color: Option<crate::types::Color>,
    #[serde(default = "default_reset_orientation_when_frozen")]
    reset_orientation_when_frozen: bool,
    #[serde(default = "default_rotation_speed")]
    rotation_speed: f64,
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_connects_to_other_radars() -> bool {
    true
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_energy_fraction_to_connect() -> f32 {
    0.9
}
fn default_energy_fraction_to_disconnect() -> f32 {
    0.1
}
fn default_is_military_target() -> bool {
    true
}
fn default_reset_orientation_when_frozen() -> bool {
    false
}
fn default_rotation_speed() -> f64 {
    0.0
}
