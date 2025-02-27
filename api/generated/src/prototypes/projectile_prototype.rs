#[derive(Debug, serde::Deserialize)]
pub struct ProjectilePrototype {
    #[serde(flatten)]
    base_: crate::prototypes::EntityPrototype,
    acceleration: f64,
    action: Option<crate::types::Trigger>,
    animation: Option<crate::types::RotatedAnimationVariations>,
    #[serde(default = "default_direction_only")]
    direction_only: bool,
    #[serde(default = "default_enable_drawing_with_mask")]
    enable_drawing_with_mask: bool,
    final_action: Option<crate::types::Trigger>,
    #[serde(default = "default_force_condition")]
    force_condition: crate::types::ForceCondition,
    #[serde(default = "default_height")]
    height: f64,
    #[serde(default = "default_hit_at_collision_position")]
    hit_at_collision_position: bool,
    hit_collision_mask: Option<crate::types::CollisionMaskConnector>,
    light: Option<crate::types::LightDefinition>,
    // default: MAX_DOUBLE
    max_speed: Option<f64>,
    #[serde(default = "default_piercing_damage")]
    piercing_damage: f32,
    #[serde(default = "default_rotatable")]
    rotatable: bool,
    shadow: Option<crate::types::RotatedAnimationVariations>,
    smoke: Option<crate::vec::Vec<crate::types::SmokeSource>>,
    // default: `{1, 1}`
    speed_modifier: Option<crate::types::Vector>,
    #[serde(default = "default_turn_speed")]
    turn_speed: f32,
    #[serde(default = "default_turning_speed_increases_exponentially_with_projectile_speed")]
    turning_speed_increases_exponentially_with_projectile_speed: bool,
}
fn default_direction_only() -> bool {
    false
}
fn default_enable_drawing_with_mask() -> bool {
    false
}
fn default_force_condition() -> crate::types::ForceCondition {
    crate::types::ForceCondition::All
}
fn default_height() -> f64 {
    1.0
}
fn default_hit_at_collision_position() -> bool {
    false
}
fn default_piercing_damage() -> f32 {
    0.0
}
fn default_rotatable() -> bool {
    true
}
fn default_turn_speed() -> f32 {
    1.0
}
fn default_turning_speed_increases_exponentially_with_projectile_speed() -> bool {
    false
}
