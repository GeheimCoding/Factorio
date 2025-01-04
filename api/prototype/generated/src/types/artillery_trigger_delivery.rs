pub struct ArtilleryTriggerDelivery {
    direction_deviation: f32,
    projectile: crate::types::EntityID,
    range_deviation: f32,
    starting_speed: f32,
    starting_speed_deviation: f32,
    trigger_fired_artillery: bool,
    type_: Artillery,
}
