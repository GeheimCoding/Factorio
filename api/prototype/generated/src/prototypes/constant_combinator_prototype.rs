#[derive(Debug, serde::Deserialize)]
pub struct ConstantCombinatorPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    activity_led_light: Option<crate::types::LightDefinition>,
    activity_led_light_offsets: (
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
        Box<crate::types::Vector>,
    ),
    activity_led_sprites: Option<crate::types::Sprite4Way>,
    circuit_wire_connection_points: (
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
        Box<crate::types::WireConnectionPoint>,
    ),
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    #[serde(default = "default_pulse_duration")]
    pulse_duration: u32,
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
fn default_pulse_duration() -> u32 {
    0
}
