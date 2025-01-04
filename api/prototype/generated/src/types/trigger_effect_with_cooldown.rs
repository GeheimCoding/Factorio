pub struct TriggerEffectWithCooldown {
    distance_cooldown: f64,
    effect: crate::types::TriggerEffect,
    initial_distance_cooldown: f64,
    initial_time_cooldown: crate::types::MapTick,
    time_cooldown: crate::types::MapTick,
}
