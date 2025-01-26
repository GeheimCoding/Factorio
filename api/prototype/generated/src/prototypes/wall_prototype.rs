#[derive(Debug, serde::Deserialize)]
pub struct WallPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: Option<crate::types::CircuitConnectorDefinition>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    connected_gate_visualization: Option<crate::types::Sprite>,
    default_output_signal: Option<crate::types::SignalIDConnector>,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    pictures: Option<WallPictures>,
    #[serde(default = "default_visual_merge_group")]
    visual_merge_group: u32,
    wall_diode_green: Option<crate::types::Sprite4Way>,
    wall_diode_green_light_bottom: Option<crate::types::LightDefinition>,
    wall_diode_green_light_left: Option<crate::types::LightDefinition>,
    wall_diode_green_light_right: Option<crate::types::LightDefinition>,
    wall_diode_green_light_top: Option<crate::types::LightDefinition>,
    wall_diode_red: Option<crate::types::Sprite4Way>,
    wall_diode_red_light_bottom: Option<crate::types::LightDefinition>,
    wall_diode_red_light_left: Option<crate::types::LightDefinition>,
    wall_diode_red_light_right: Option<crate::types::LightDefinition>,
    wall_diode_red_light_top: Option<crate::types::LightDefinition>,
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
pub struct WallPictures {
    corner_left_down: Option<crate::types::SpriteVariations>,
    corner_right_down: Option<crate::types::SpriteVariations>,
    ending_left: Option<crate::types::SpriteVariations>,
    ending_right: Option<crate::types::SpriteVariations>,
    filling: Option<crate::types::SpriteVariations>,
    gate_connection_patch: Option<crate::types::Sprite4Way>,
    single: Option<crate::types::SpriteVariations>,
    straight_horizontal: Option<crate::types::SpriteVariations>,
    straight_vertical: Option<crate::types::SpriteVariations>,
    t_up: Option<crate::types::SpriteVariations>,
    water_connection_patch: Option<crate::types::Sprite4Way>,
}
fn default_visual_merge_group() -> u32 {
    0
}
