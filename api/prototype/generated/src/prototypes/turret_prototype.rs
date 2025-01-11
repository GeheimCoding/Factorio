pub struct TurretPrototype {
    alert_when_attacking: bool,
    allow_turning_when_starting_attack: bool,
    attack_from_start_frame: bool,
    attack_parameters: crate::types::AttackParameters,
    attack_target_mask: crate::types::TriggerTargetMask,
    attacking_animation: crate::types::RotatedAnimation8Way,
    attacking_speed: f32,
    call_for_help_radius: f64,
    circuit_connector: Vec<crate::types::CircuitConnectorDefinition>,
    circuit_wire_max_distance: f64,
    default_speed: f32,
    default_speed_secondary: f32,
    default_speed_when_killed: f32,
    default_starting_progress_when_killed: f32,
    draw_circuit_wires: bool,
    draw_copper_wires: bool,
    dying_sound: crate::types::Sound,
    ending_attack_animation: crate::types::RotatedAnimation8Way,
    ending_attack_speed: f32,
    ending_attack_speed_secondary: f32,
    ending_attack_speed_when_killed: f32,
    ending_attack_starting_progress_when_killed: f32,
    energy_glow_animation: crate::types::RotatedAnimation8Way,
    energy_glow_animation_flicker_strength: f32,
    folded_animation: crate::types::RotatedAnimation8Way,
    folded_animation_is_stateless: bool,
    folded_speed: f32,
    folded_speed_secondary: f32,
    folded_speed_when_killed: f32,
    folded_starting_progress_when_killed: f32,
    folded_state_corpse: TurretPrototypeFoldedStateCorpse,
    folding_animation: crate::types::RotatedAnimation8Way,
    folding_sound: crate::types::Sound,
    folding_speed: f32,
    folding_speed_secondary: f32,
    folding_speed_when_killed: f32,
    folding_starting_progress_when_killed: f32,
    glow_light_intensity: f32,
    graphics_set: crate::types::TurretGraphicsSet,
    gun_animation_render_layer: crate::types::RenderLayer,
    gun_animation_secondary_draw_order: u8,
    ignore_target_mask: crate::types::TriggerTargetMask,
    integration: crate::types::Sprite,
    is_military_target: bool,
    prepare_range: f64,
    prepared_alternative_animation: crate::types::RotatedAnimation8Way,
    prepared_alternative_chance: f32,
    prepared_alternative_sound: crate::types::Sound,
    prepared_alternative_speed: f32,
    prepared_alternative_speed_secondary: f32,
    prepared_alternative_speed_when_killed: f32,
    prepared_alternative_starting_progress_when_killed: f32,
    prepared_animation: crate::types::RotatedAnimation8Way,
    prepared_sound: crate::types::Sound,
    prepared_speed: f32,
    prepared_speed_secondary: f32,
    prepared_speed_when_killed: f32,
    prepared_starting_progress_when_killed: f32,
    preparing_animation: crate::types::RotatedAnimation8Way,
    preparing_sound: crate::types::Sound,
    preparing_speed: f32,
    preparing_speed_secondary: f32,
    preparing_speed_when_killed: f32,
    preparing_starting_progress_when_killed: f32,
    random_animation_offset: bool,
    resource_indicator_animation: crate::types::RotatedAnimation8Way,
    rotating_sound: crate::types::InterruptibleSound,
    rotation_speed: f32,
    rotation_speed_secondary: f32,
    rotation_speed_when_killed: f32,
    rotation_starting_progress_when_killed: f32,
    shoot_in_prepare_state: bool,
    spawn_decoration: Vec<crate::types::CreateDecorativesTriggerEffectItem>,
    spawn_decorations_on_expansion: bool,
    special_effect: crate::types::TurretSpecialEffect,
    start_attacking_only_when_can_shoot: bool,
    starting_attack_animation: crate::types::RotatedAnimation8Way,
    starting_attack_sound: crate::types::Sound,
    starting_attack_speed: f32,
    starting_attack_speed_secondary: f32,
    starting_attack_speed_when_killed: f32,
    starting_attack_starting_progress_when_killed: f32,
    turret_base_has_direction: bool,
    unfolds_before_dying: bool,
}
pub enum TurretPrototypeFoldedStateCorpse {
    EntityID(crate::types::EntityID),
    VecEntityID(Vec<crate::types::EntityID>),
}