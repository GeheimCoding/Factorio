#[derive(Debug, serde::Deserialize)]
pub struct ConstantCombinatorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    activity_led_light: Option<crate::types::LightDefinition>,
    activity_led_light_offsets: (
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
        crate::types::Vector,
    ),
    activity_led_sprites: Option<crate::types::Sprite4Way>,
    circuit_wire_connection_points: (
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
        crate::types::WireConnectionPoint,
    ),
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    sprites: Option<crate::types::Sprite4Way>,
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
