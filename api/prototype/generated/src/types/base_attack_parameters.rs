#[derive(serde::Deserialize)]
pub struct BaseAttackParameters {
    #[serde(default = "default_activation_type")]
    activation_type: BaseAttackParametersActivationType,
    ammo_categories: Vec<crate::types::AmmoCategoryID>,
    ammo_category: crate::types::AmmoCategoryID,
    #[serde(default = "default_ammo_consumption_modifier")]
    ammo_consumption_modifier: f32,
    ammo_type: crate::types::AmmoType,
    animation: crate::types::RotatedAnimation,
    cooldown: f32,
    #[serde(default = "default_cooldown_deviation")]
    cooldown_deviation: f32,
    cyclic_sound: crate::types::CyclicSound,
    #[serde(default = "default_damage_modifier")]
    damage_modifier: f32,
    #[serde(default = "default_fire_penalty")]
    fire_penalty: f32,
    #[serde(default = "default_health_penalty")]
    health_penalty: f32,
    #[serde(default = "default_lead_target_for_projectile_delay")]
    lead_target_for_projectile_delay: u32,
    #[serde(default = "default_lead_target_for_projectile_speed")]
    lead_target_for_projectile_speed: f32,
    // default: equal to `range` property
    min_attack_distance: f32,
    #[serde(default = "default_min_range")]
    min_range: f32,
    // default: equal to `cooldown` property
    movement_slow_down_cooldown: f32,
    #[serde(default = "default_movement_slow_down_factor")]
    movement_slow_down_factor: f64,
    range: f32,
    #[serde(default = "default_range_mode")]
    range_mode: crate::types::RangeMode,
    #[serde(default = "default_rotate_penalty")]
    rotate_penalty: f32,
    sound: crate::types::LayeredSound,
    #[serde(default = "default_turn_range")]
    turn_range: f32,
    #[serde(default = "default_use_shooter_direction")]
    use_shooter_direction: bool,
    #[serde(default = "default_warmup")]
    warmup: u32,
}
#[derive(serde::Deserialize)]
pub enum BaseAttackParametersActivationType {
    #[serde(rename = "shoot")]
    Shoot,
    #[serde(rename = "throw")]
    Throw,
    #[serde(rename = "consume")]
    Consume,
    #[serde(rename = "activate")]
    Activate,
}
fn default_activation_type() -> BaseAttackParametersActivationType {
    BaseAttackParametersActivationType::Shoot
}
fn default_ammo_consumption_modifier() -> f32 {
    1.0
}
fn default_cooldown_deviation() -> f32 {
    0.0
}
fn default_damage_modifier() -> f32 {
    1.0
}
fn default_fire_penalty() -> f32 {
    0.0
}
fn default_health_penalty() -> f32 {
    0.0
}
fn default_lead_target_for_projectile_delay() -> u32 {
    0
}
fn default_lead_target_for_projectile_speed() -> f32 {
    0.0
}
fn default_min_range() -> f32 {
    0.0
}
fn default_movement_slow_down_factor() -> f64 {
    1.0
}
fn default_range_mode() -> crate::types::RangeMode {
    crate::types::RangeMode::CenterToCenter
}
fn default_rotate_penalty() -> f32 {
    0.0
}
fn default_turn_range() -> f32 {
    1.0
}
fn default_use_shooter_direction() -> bool {
    false
}
fn default_warmup() -> u32 {
    0
}
