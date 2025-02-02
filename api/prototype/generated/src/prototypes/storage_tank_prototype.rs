#[derive(Debug, serde::Deserialize)]
pub struct StorageTankPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: Option<(
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
        crate::types::CircuitConnectorDefinition,
    )>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    flow_length_in_ticks: u32,
    fluid_box: crate::types::FluidBox,
    pictures: Option<StorageTankPictures>,
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
#[derive(Debug, serde::Deserialize)]
pub struct StorageTankPictures {
    flow_sprite: Option<crate::types::Sprite>,
    fluid_background: Option<crate::types::Sprite>,
    frozen_patch: Option<crate::types::Sprite4Way>,
    gas_flow: Option<crate::types::Animation>,
    picture: Option<crate::types::Sprite4Way>,
    window_background: Option<crate::types::Sprite>,
}
fn default_show_fluid_icon() -> bool {
    true
}
fn default_two_direction_only() -> bool {
    false
}
