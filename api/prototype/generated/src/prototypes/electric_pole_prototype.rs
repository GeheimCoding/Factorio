#[derive(Debug, serde::Deserialize)]
pub struct ElectricPolePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    active_picture: Option<crate::types::Sprite>,
    #[serde(default = "default_auto_connect_up_to_n_wires")]
    auto_connect_up_to_n_wires: u8,
    connection_points: Vec<crate::types::WireConnectionPoint>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_maximum_wire_distance")]
    maximum_wire_distance: f64,
    pictures: Option<crate::types::RotatedSprite>,
    radius_visualisation_picture: Option<crate::types::Sprite>,
    supply_area_distance: f64,
    #[serde(default = "default_track_coverage_during_build_by_moving")]
    track_coverage_during_build_by_moving: bool,
}
fn default_auto_connect_up_to_n_wires() -> u8 {
    5
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_maximum_wire_distance() -> f64 {
    0.0
}
fn default_track_coverage_during_build_by_moving() -> bool {
    false
}
