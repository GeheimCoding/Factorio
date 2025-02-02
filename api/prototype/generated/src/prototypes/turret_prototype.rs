#[derive(Debug, serde::Deserialize)]
pub struct TurretPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityWithOwnerPrototype,
    #[serde(default = "default_alert_when_attacking")]
    alert_when_attacking: bool,
    #[serde(default = "default_allow_turning_when_starting_attack")]
    allow_turning_when_starting_attack: bool,
    #[serde(default = "default_attack_from_start_frame")]
    attack_from_start_frame: bool,
    attack_parameters: crate::types::AttackParameters,
    // default: all masks
    attack_target_mask: Option<crate::types::TriggerTargetMask>,
    attacking_animation: Option<crate::types::RotatedAnimation8Way>,
    #[serde(default = "default_attacking_speed")]
    attacking_speed: f32,
    call_for_help_radius: f64,
    circuit_connector: Option<crate::vec::Vec<crate::types::CircuitConnectorDefinition>>,
    #[serde(default = "default_circuit_wire_max_distance")]
    circuit_wire_max_distance: f64,
    #[serde(default = "default_default_speed")]
    default_speed: f32,
    // default: Value of `default_speed`
    default_speed_secondary: Option<f32>,
    // default: Value of `default_speed`
    default_speed_when_killed: Option<f32>,
    #[serde(default = "default_default_starting_progress_when_killed")]
    default_starting_progress_when_killed: f32,
    #[serde(default = "default_draw_circuit_wires")]
    draw_circuit_wires: bool,
    #[serde(default = "default_draw_copper_wires")]
    draw_copper_wires: bool,
    dying_sound: Option<crate::types::Sound>,
    ending_attack_animation: Option<crate::types::RotatedAnimation8Way>,
    // default: Value of `default_speed`
    ending_attack_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    ending_attack_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    ending_attack_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    ending_attack_starting_progress_when_killed: Option<f32>,
    energy_glow_animation: Option<crate::types::RotatedAnimation8Way>,
    #[serde(default = "default_energy_glow_animation_flicker_strength")]
    energy_glow_animation_flicker_strength: f32,
    folded_animation: crate::types::RotatedAnimation8Way,
    #[serde(default = "default_folded_animation_is_stateless")]
    folded_animation_is_stateless: bool,
    // default: Value of `default_speed`
    folded_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    folded_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    folded_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    folded_starting_progress_when_killed: Option<f32>,
    folded_state_corpse: Option<TurretPrototypeFoldedStateCorpse>,
    folding_animation: Option<crate::types::RotatedAnimation8Way>,
    folding_sound: Option<crate::types::Sound>,
    // default: Value of `default_speed`
    folding_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    folding_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    folding_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    folding_starting_progress_when_killed: Option<f32>,
    #[serde(default = "default_glow_light_intensity")]
    glow_light_intensity: f32,
    graphics_set: crate::types::TurretGraphicsSet,
    #[serde(default = "default_gun_animation_render_layer")]
    gun_animation_render_layer: crate::types::RenderLayer,
    #[serde(default = "default_gun_animation_secondary_draw_order")]
    gun_animation_secondary_draw_order: u8,
    // default: no masks
    ignore_target_mask: Option<crate::types::TriggerTargetMask>,
    integration: Option<crate::types::Sprite>,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    // default: The range defined in the `attack_parameters`
    prepare_range: Option<f64>,
    prepared_alternative_animation: Option<crate::types::RotatedAnimation8Way>,
    #[serde(default = "default_prepared_alternative_chance")]
    prepared_alternative_chance: f32,
    prepared_alternative_sound: Option<crate::types::Sound>,
    // default: Value of `default_speed`
    prepared_alternative_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    prepared_alternative_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    prepared_alternative_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    prepared_alternative_starting_progress_when_killed: Option<f32>,
    prepared_animation: Option<crate::types::RotatedAnimation8Way>,
    prepared_sound: Option<crate::types::Sound>,
    // default: Value of `default_speed`
    prepared_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    prepared_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    prepared_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    prepared_starting_progress_when_killed: Option<f32>,
    preparing_animation: Option<crate::types::RotatedAnimation8Way>,
    preparing_sound: Option<crate::types::Sound>,
    // default: Value of `default_speed`
    preparing_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    preparing_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    preparing_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    preparing_starting_progress_when_killed: Option<f32>,
    #[serde(default = "default_random_animation_offset")]
    random_animation_offset: bool,
    resource_indicator_animation: Option<crate::types::RotatedAnimation8Way>,
    rotating_sound: Option<crate::types::InterruptibleSound>,
    // default: Value of `default_speed`
    rotation_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    rotation_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    rotation_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    rotation_starting_progress_when_killed: Option<f32>,
    #[serde(default = "default_shoot_in_prepare_state")]
    shoot_in_prepare_state: bool,
    spawn_decoration: Option<crate::vec::Vec<crate::types::CreateDecorativesTriggerEffectItem>>,
    #[serde(default = "default_spawn_decorations_on_expansion")]
    spawn_decorations_on_expansion: bool,
    special_effect: Option<crate::types::TurretSpecialEffect>,
    #[serde(default = "default_start_attacking_only_when_can_shoot")]
    start_attacking_only_when_can_shoot: bool,
    starting_attack_animation: Option<crate::types::RotatedAnimation8Way>,
    starting_attack_sound: Option<crate::types::Sound>,
    // default: Value of `default_speed`
    starting_attack_speed: Option<f32>,
    // default: Value of `default_speed_secondary`
    starting_attack_speed_secondary: Option<f32>,
    // default: Value of `default_speed_when_killed`
    starting_attack_speed_when_killed: Option<f32>,
    // default: Value of `default_starting_progress_when_killed`
    starting_attack_starting_progress_when_killed: Option<f32>,
    #[serde(default = "default_turret_base_has_direction")]
    turret_base_has_direction: bool,
    #[serde(default = "default_unfolds_before_dying")]
    unfolds_before_dying: bool,
}
fn default_alert_when_attacking() -> bool {
    true
}
fn default_allow_turning_when_starting_attack() -> bool {
    false
}
fn default_attack_from_start_frame() -> bool {
    false
}
fn default_attacking_speed() -> f32 {
    1.0
}
fn default_circuit_wire_max_distance() -> f64 {
    0.0
}
fn default_default_speed() -> f32 {
    1.0
}
fn default_default_starting_progress_when_killed() -> f32 {
    0.0
}
fn default_draw_circuit_wires() -> bool {
    true
}
fn default_draw_copper_wires() -> bool {
    true
}
fn default_energy_glow_animation_flicker_strength() -> f32 {
    0.2
}
fn default_folded_animation_is_stateless() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum TurretPrototypeFoldedStateCorpse {
    #[serde(untagged)]
    EntityID(crate::types::EntityID),
    #[serde(untagged)]
    VecEntityID(crate::vec::Vec<crate::types::EntityID>),
}
fn default_glow_light_intensity() -> f32 {
    0.0
}
fn default_gun_animation_render_layer() -> crate::types::RenderLayer {
    crate::types::RenderLayer::Object
}
fn default_gun_animation_secondary_draw_order() -> u8 {
    0
}
fn default_is_military_target() -> bool {
    true
}
fn default_prepared_alternative_chance() -> f32 {
    0.0
}
fn default_random_animation_offset() -> bool {
    false
}
fn default_shoot_in_prepare_state() -> bool {
    false
}
fn default_spawn_decorations_on_expansion() -> bool {
    false
}
fn default_start_attacking_only_when_can_shoot() -> bool {
    false
}
fn default_turret_base_has_direction() -> bool {
    false
}
fn default_unfolds_before_dying() -> bool {
    false
}
