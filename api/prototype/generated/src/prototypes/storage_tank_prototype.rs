#[derive(serde::Deserialize)]
pub struct StorageTankPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: (
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    ),
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    flow_length_in_ticks: u32,
    fluid_box: crate::types::FluidBox,
    pictures: StorageTankPictures,
    #[serde(default = "default_show_fluid_icon")]
    show_fluid_icon: bool,
    #[serde(default = "default_two_direction_only")]
    two_direction_only: bool,
    window_bounding_box: crate::types::BoundingBox,
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
#[derive(serde::Deserialize)]
pub struct StorageTankPictures {
    flow_sprite: crate::types::Sprite,
    fluid_background: crate::types::Sprite,
    frozen_patch: crate::types::Sprite4Way,
    gas_flow: crate::types::Animation,
    picture: crate::types::Sprite4Way,
    window_background: crate::types::Sprite,
}
fn default_show_fluid_icon() -> bool {
    true
}
fn default_two_direction_only() -> bool {
    false
}
