#[derive(Debug, serde::Deserialize)]
pub struct CranePart {
    #[serde(default = "default_allow_sprite_rotation")]
    allow_sprite_rotation: bool,
    dying_effect: Option<crate::types::CranePartDyingEffect>,
    extendable_length: Option<crate::types::Vector3D>,
    extendable_length_grappler: Option<crate::types::Vector3D>,
    #[serde(default = "default_is_contractible_by_cropping")]
    is_contractible_by_cropping: bool,
    #[serde(default = "default_layer")]
    layer: i8,
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_orientation_shift")]
    orientation_shift: f32,
    relative_position: Option<crate::types::Vector3D>,
    relative_position_grappler: Option<crate::types::Vector3D>,
    rotated_sprite: Option<crate::types::RotatedSprite>,
    rotated_sprite_reflection: Option<crate::types::RotatedSprite>,
    rotated_sprite_shadow: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_scale_to_fit_model")]
    scale_to_fit_model: bool,
    #[serde(default = "default_should_scale_for_perspective")]
    should_scale_for_perspective: bool,
    #[serde(default = "default_snap_end")]
    snap_end: f32,
    #[serde(default = "default_snap_end_arm_extent_multiplier")]
    snap_end_arm_extent_multiplier: f32,
    #[serde(default = "default_snap_start")]
    snap_start: f32,
    sprite: Option<crate::types::Sprite>,
    sprite_reflection: Option<crate::types::Sprite>,
    sprite_shadow: Option<crate::types::Sprite>,
    static_length: Option<crate::types::Vector3D>,
    static_length_grappler: Option<crate::types::Vector3D>,
}
fn default_allow_sprite_rotation() -> bool {
    true
}
fn default_is_contractible_by_cropping() -> bool {
    false
}
fn default_layer() -> i8 {
    0
}
fn default_name() -> String {
    String::from("")
}
fn default_orientation_shift() -> f32 {
    0.0
}
fn default_scale_to_fit_model() -> bool {
    false
}
fn default_should_scale_for_perspective() -> bool {
    true
}
fn default_snap_end() -> f32 {
    0.0
}
fn default_snap_end_arm_extent_multiplier() -> f32 {
    0.0
}
fn default_snap_start() -> f32 {
    0.0
}
