#[derive(Debug, serde::Deserialize)]
pub struct GatePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    activation_distance: f64,
    closing_sound: Option<crate::types::Sound>,
    #[serde(default = "default_fadeout_interval")]
    fadeout_interval: u32,
    horizontal_animation: Option<crate::types::Animation>,
    horizontal_rail_animation_left: Option<crate::types::Animation>,
    horizontal_rail_animation_right: Option<crate::types::Animation>,
    horizontal_rail_base: Option<crate::types::Animation>,
    opened_collision_mask: Option<crate::types::CollisionMaskConnector>,
    opening_sound: Option<crate::types::Sound>,
    opening_speed: f32,
    timeout_to_close: u32,
    vertical_animation: Option<crate::types::Animation>,
    vertical_rail_animation_left: Option<crate::types::Animation>,
    vertical_rail_animation_right: Option<crate::types::Animation>,
    vertical_rail_base: Option<crate::types::Animation>,
    wall_patch: Option<crate::types::Animation>,
}
fn default_fadeout_interval() -> u32 {
    0
}
