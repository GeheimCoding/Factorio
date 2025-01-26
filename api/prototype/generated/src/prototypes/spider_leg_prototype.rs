#[derive(Debug, serde::Deserialize)]
pub struct SpiderLegPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_ankle_height")]
    ankle_height: f64,
    base_position_selection_distance: f64,
    graphics_set: Option<SpiderLegGraphicsSet>,
    #[serde(default = "default_hip_flexibility")]
    hip_flexibility: f64,
    initial_movement_speed: f64,
    knee_distance_factor: f64,
    knee_height: f64,
    lower_leg_dying_trigger_effects: Option<Vec<crate::types::SpiderLegTriggerEffect>>,
    minimal_step_size: f64,
    movement_acceleration: f64,
    movement_based_position_selection_distance: f64,
    #[serde(default = "default_stretch_force_scalar")]
    stretch_force_scalar: f64,
    target_position_randomisation_distance: f64,
    upper_leg_dying_trigger_effects: Option<Vec<crate::types::SpiderLegTriggerEffect>>,
    #[serde(default = "default_walking_sound_speed_modifier")]
    walking_sound_speed_modifier: f32,
    #[serde(default = "default_walking_sound_volume_modifier")]
    walking_sound_volume_modifier: f32,
}
fn default_ankle_height() -> f64 {
    0.0
}
#[derive(Debug, serde::Deserialize)]
pub struct SpiderLegGraphicsSet {
    foot: Option<crate::types::RotatedSprite>,
    foot_shadow: Option<crate::types::RotatedSprite>,
    joint: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_joint_render_layer")]
    joint_render_layer: crate::types::RenderLayer,
    joint_shadow: Option<crate::types::RotatedSprite>,
    #[serde(default = "default_joint_turn_offset")]
    joint_turn_offset: f32,
    lower_part: Option<crate::types::SpiderLegPart>,
    lower_part_shadow: Option<crate::types::SpiderLegPart>,
    lower_part_water_reflection: Option<crate::types::SpiderLegPart>,
    upper_part: Option<crate::types::SpiderLegPart>,
    upper_part_shadow: Option<crate::types::SpiderLegPart>,
    upper_part_water_reflection: Option<crate::types::SpiderLegPart>,
}
fn default_joint_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::TrainStopTop
}
fn default_joint_turn_offset() -> f32 {
    0.0
}
fn default_hip_flexibility() -> f64 {
    1.0
}
fn default_stretch_force_scalar() -> f64 {
    0.7
}
fn default_walking_sound_speed_modifier() -> f32 {
    1.0
}
fn default_walking_sound_volume_modifier() -> f32 {
    1.0
}
