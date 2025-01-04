pub struct ProjectileTriggerDelivery {
    direction_deviation: f32,
    max_range: f64,
    min_range: f64,
    projectile: crate::types::EntityID,
    range_deviation: f32,
    starting_speed: f32,
    starting_speed_deviation: f32,
    type_: Projectile,
}
