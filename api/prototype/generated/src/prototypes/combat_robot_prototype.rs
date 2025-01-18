#[derive(serde::Deserialize)]
pub struct CombatRobotPrototype {
    base_: crate::prototypes::FlyingRobotPrototype,
    attack_parameters: crate::types::AttackParameters,
    destroy_action: crate::types::Trigger,
    #[serde(default = "default_follows_player")]
    follows_player: bool,
    #[serde(default = "default_friction")]
    friction: f64,
    idle: crate::types::RotatedAnimation,
    in_motion: crate::types::RotatedAnimation,
    light: crate::types::LightDefinition,
    #[serde(default = "default_range_from_player")]
    range_from_player: f64,
    shadow_idle: crate::types::RotatedAnimation,
    shadow_in_motion: crate::types::RotatedAnimation,
    time_to_live: u32,
}
fn default_follows_player() -> bool {
    false
}
fn default_friction() -> f64 {
    0.0
}
fn default_range_from_player() -> f64 {
    0.0
}
