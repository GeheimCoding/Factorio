#[derive(Debug, serde::Deserialize)]
pub struct UnitPrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    absorptions_to_join_attack:
        Option<std::collections::HashMap<crate::types::AirbornePollutantID, f32>>,
    #[serde(default = "default_affected_by_tiles")]
    affected_by_tiles: bool,
    ai_settings: Option<crate::types::UnitAISettings>,
    #[serde(default = "default_allow_run_time_change_of_is_military_target")]
    allow_run_time_change_of_is_military_target: bool,
    alternative_attacking_frame_sequence: Option<crate::types::UnitAlternativeFrameSequence>,
    attack_parameters: crate::types::AttackParameters,
    #[serde(default = "default_can_open_gates")]
    can_open_gates: bool,
    distance_per_frame: f32,
    distraction_cooldown: u32,
    dying_sound: Option<crate::types::Sound>,
    #[serde(default = "default_has_belt_immunity")]
    has_belt_immunity: bool,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    light: Option<crate::types::LightDefinition>,
    #[serde(default = "default_max_pursue_distance")]
    max_pursue_distance: f64,
    #[serde(default = "default_min_pursue_time")]
    min_pursue_time: u32,
    #[serde(default = "default_move_while_shooting")]
    move_while_shooting: bool,
    movement_speed: f32,
    #[serde(default = "default_radar_range")]
    radar_range: u32,
    #[serde(default = "default_render_layer")]
    render_layer: crate::types::RenderLayer,
    #[serde(default = "default_rotation_speed")]
    rotation_speed: f32,
    run_animation: crate::types::RotatedAnimation,
    running_sound_animation_positions: Option<Vec<f32>>,
    #[serde(default = "default_spawning_time_modifier")]
    spawning_time_modifier: f64,
    vision_distance: f64,
    walking_sound: Option<crate::types::Sound>,
    warcry: Option<crate::types::Sound>,
}
fn default_affected_by_tiles() -> bool {
    false
}
fn default_allow_run_time_change_of_is_military_target() -> bool {
    false
}
fn default_can_open_gates() -> bool {
    false
}
fn default_has_belt_immunity() -> bool {
    false
}
fn default_is_military_target() -> bool {
    true
}
fn default_max_pursue_distance() -> f64 {
    50.0
}
fn default_min_pursue_time() -> u32 {
    600
}
fn default_move_while_shooting() -> bool {
    false
}
fn default_radar_range() -> u32 {
    0
}
fn default_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_rotation_speed() -> f32 {
    0.0
}
fn default_spawning_time_modifier() -> f64 {
    1.0
}
