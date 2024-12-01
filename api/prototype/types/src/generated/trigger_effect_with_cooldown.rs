pub struct TriggerEffectWithCooldown {
    distance_cooldown: f64,
    effect: TriggerEffect,
    initial_distance_cooldown: f64,
    initial_time_cooldown: MapTick,
    time_cooldown: MapTick,
}
