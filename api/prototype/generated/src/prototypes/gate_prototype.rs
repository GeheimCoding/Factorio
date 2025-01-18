#[derive(serde::Deserialize)]
pub struct GatePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    activation_distance: f64,
    closing_sound: crate::types::Sound,
    #[serde(default = "default_fadeout_interval")]
    fadeout_interval: u32,
    horizontal_animation: crate::types::Animation,
    horizontal_rail_animation_left: crate::types::Animation,
    horizontal_rail_animation_right: crate::types::Animation,
    horizontal_rail_base: crate::types::Animation,
    opened_collision_mask: crate::types::CollisionMaskConnector,
    opening_sound: crate::types::Sound,
    opening_speed: f32,
    timeout_to_close: u32,
    vertical_animation: crate::types::Animation,
    vertical_rail_animation_left: crate::types::Animation,
    vertical_rail_animation_right: crate::types::Animation,
    vertical_rail_base: crate::types::Animation,
    wall_patch: crate::types::Animation,
}
fn default_fadeout_interval() -> u32 {
    0
}
