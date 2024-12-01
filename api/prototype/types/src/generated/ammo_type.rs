pub struct AmmoType {
    action: Trigger,
    clamp_position: bool,
    consumption_modifier: f32,
    cooldown_modifier: f64,
    energy_consumption: Energy,
    range_modifier: f64,
    source_type: AmmoSourceType,
    target_filter: Vec<EntityID>,
    target_type: AmmoTypeTargetType,
}
pub enum AmmoTypeTargetType {
    Entity,
    Position,
    Direction,
}
