pub struct AreaTriggerItem {
    base_: crate::types::TriggerItem,
    collision_mode: AreaTriggerItemCollisionMode,
    radius: f64,
    show_in_tooltip: bool,
    target_entities: bool,
    trigger_from_target: bool,
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum AreaTriggerItemCollisionMode {
    #[serde(rename = "distance_from_collision_box")]
    DistanceFromCollisionBox,
    #[serde(rename = "distance_from_center")]
    DistanceFromCenter,
}
