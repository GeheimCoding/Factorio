#[derive(Debug, serde::Deserialize)]
pub struct CreateParticleTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    apply_tile_tint: Option<CreateParticleTriggerEffectItemApplyTileTint>,
    #[serde(default = "default_frame_speed")]
    frame_speed: f32,
    #[serde(default = "default_frame_speed_deviation")]
    frame_speed_deviation: f32,
    initial_height: f32,
    #[serde(default = "default_initial_height_deviation")]
    initial_height_deviation: f32,
    #[serde(default = "default_initial_vertical_speed")]
    initial_vertical_speed: f32,
    #[serde(default = "default_initial_vertical_speed_deviation")]
    initial_vertical_speed_deviation: f32,
    #[serde(default = "default_movement_multiplier")]
    movement_multiplier: f32,
    offset_deviation: Option<crate::types::SimpleBoundingBox>,
    offsets: Option<Vec<crate::types::Vector>>,
    #[serde(default = "default_only_when_visible")]
    only_when_visible: bool,
    particle_name: crate::types::ParticleID,
    #[serde(default = "default_rotate_offsets")]
    rotate_offsets: bool,
    #[serde(default = "default_show_in_tooltip")]
    show_in_tooltip: bool,
    #[serde(default = "default_speed_from_center")]
    speed_from_center: f32,
    #[serde(default = "default_speed_from_center_deviation")]
    speed_from_center_deviation: f32,
    #[serde(default = "default_tail_length")]
    tail_length: u8,
    #[serde(default = "default_tail_length_deviation")]
    tail_length_deviation: u8,
    #[serde(default = "default_tail_width")]
    tail_width: f32,
    tile_collision_mask: Option<crate::types::CollisionMaskConnector>,
    // default: `{1, 1, 1, 1} (white)`
    tint: Option<crate::types::Color>,
}
#[derive(Debug, serde::Deserialize)]
pub enum CreateParticleTriggerEffectItemApplyTileTint {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
}
fn default_frame_speed() -> f32 {
    1.0
}
fn default_frame_speed_deviation() -> f32 {
    0.0
}
fn default_initial_height_deviation() -> f32 {
    0.0
}
fn default_initial_vertical_speed() -> f32 {
    0.0
}
fn default_initial_vertical_speed_deviation() -> f32 {
    0.0
}
fn default_movement_multiplier() -> f32 {
    0.0
}
fn default_only_when_visible() -> bool {
    false
}
fn default_rotate_offsets() -> bool {
    false
}
fn default_show_in_tooltip() -> bool {
    false
}
fn default_speed_from_center() -> f32 {
    0.0
}
fn default_speed_from_center_deviation() -> f32 {
    0.0
}
fn default_tail_length() -> u8 {
    0
}
fn default_tail_length_deviation() -> u8 {
    0
}
fn default_tail_width() -> f32 {
    1.0
}
