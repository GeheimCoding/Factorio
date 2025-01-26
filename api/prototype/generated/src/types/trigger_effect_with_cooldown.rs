#[derive(Debug, serde::Deserialize)]
pub struct TriggerEffectWithCooldown {
    #[serde(default = "default_distance_cooldown")]
    distance_cooldown: f64,
    effect: crate::types::TriggerEffect,
    #[serde(default = "default_initial_distance_cooldown")]
    initial_distance_cooldown: f64,
    #[serde(default = "default_initial_time_cooldown")]
    initial_time_cooldown: crate::types::MapTick,
    #[serde(default = "default_time_cooldown")]
    time_cooldown: crate::types::MapTick,
}
fn default_distance_cooldown() -> f64 {
    0.0
}
fn default_initial_distance_cooldown() -> f64 {
    0.0
}
fn default_initial_time_cooldown() -> crate::types::MapTick {
    0
}
fn default_time_cooldown() -> crate::types::MapTick {
    0
}
