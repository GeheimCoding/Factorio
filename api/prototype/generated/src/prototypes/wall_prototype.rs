#[derive(serde::Deserialize)]
pub struct WallPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    circuit_connector: crate::types::CircuitConnectorDefinition,
    circuit_wire_max_distance: f64,
    connected_gate_visualization: crate::types::Sprite,
    default_output_signal: crate::types::SignalIDConnector,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    pictures: WallPictures,
    visual_merge_group: u32,
    wall_diode_green: crate::types::Sprite4Way,
    wall_diode_green_light_bottom: crate::types::LightDefinition,
    wall_diode_green_light_left: crate::types::LightDefinition,
    wall_diode_green_light_right: crate::types::LightDefinition,
    wall_diode_green_light_top: crate::types::LightDefinition,
    wall_diode_red: crate::types::Sprite4Way,
    wall_diode_red_light_bottom: crate::types::LightDefinition,
    wall_diode_red_light_left: crate::types::LightDefinition,
    wall_diode_red_light_right: crate::types::LightDefinition,
    wall_diode_red_light_top: crate::types::LightDefinition,
}
#[derive(serde::Deserialize)]
pub struct WallPictures {
    corner_left_down: crate::types::SpriteVariations,
    corner_right_down: crate::types::SpriteVariations,
    ending_left: crate::types::SpriteVariations,
    ending_right: crate::types::SpriteVariations,
    filling: crate::types::SpriteVariations,
    gate_connection_patch: crate::types::Sprite4Way,
    single: crate::types::SpriteVariations,
    straight_horizontal: crate::types::SpriteVariations,
    straight_vertical: crate::types::SpriteVariations,
    t_up: crate::types::SpriteVariations,
    water_connection_patch: crate::types::Sprite4Way,
}
