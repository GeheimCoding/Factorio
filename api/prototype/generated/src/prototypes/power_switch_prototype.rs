#[derive(Debug, serde::Deserialize)]
pub struct PowerSwitchPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_wire_connection_point: crate::types::WireConnectionPoint,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    frozen_patch: Option<crate::types::Sprite>,
    led_off: Option<crate::types::Sprite>,
    led_on: Option<crate::types::Sprite>,
    left_wire_connection_point: crate::types::WireConnectionPoint,
    overlay_loop: Option<crate::types::Animation>,
    overlay_start: Option<crate::types::Animation>,
    overlay_start_delay: u8,
    power_on_animation: Option<crate::types::Animation>,
    right_wire_connection_point: crate::types::WireConnectionPoint,
    #[serde(default = "default_wire_max_distance")]
    wire_max_distance: f64,
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_wire_max_distance() -> f64 {
    0.0
}
