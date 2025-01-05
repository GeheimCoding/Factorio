pub struct AreaTriggerItem {
    collision_mode: AreaTriggerItemCollisionMode,
    radius: f64,
    show_in_tooltip: bool,
    target_entities: bool,
    trigger_from_target: bool,
    type_: String,
}
pub enum AreaTriggerItemCollisionMode {
    DistanceFromCollisionBox,
    DistanceFromCenter,
}
