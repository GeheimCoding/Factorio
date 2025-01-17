pub struct BaseAttackParameters {
    activation_type: BaseAttackParametersActivationType,
    ammo_categories: Vec<crate::types::AmmoCategoryID>,
    ammo_category: crate::types::AmmoCategoryID,
    ammo_consumption_modifier: f32,
    ammo_type: crate::types::AmmoType,
    animation: crate::types::RotatedAnimation,
    cooldown: f32,
    cooldown_deviation: f32,
    cyclic_sound: crate::types::CyclicSound,
    damage_modifier: f32,
    fire_penalty: f32,
    health_penalty: f32,
    lead_target_for_projectile_delay: u32,
    lead_target_for_projectile_speed: f32,
    min_attack_distance: f32,
    min_range: f32,
    movement_slow_down_cooldown: f32,
    movement_slow_down_factor: f64,
    range: f32,
    range_mode: crate::types::RangeMode,
    rotate_penalty: f32,
    sound: crate::types::LayeredSound,
    turn_range: f32,
    use_shooter_direction: bool,
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
