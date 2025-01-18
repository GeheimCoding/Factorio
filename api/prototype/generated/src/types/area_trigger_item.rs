#[derive(serde::Deserialize)]
pub struct AreaTriggerItem {
    base_: crate::types::TriggerItem,
    #[serde(default = "default_collision_mode")]
    collision_mode: AreaTriggerItemCollisionMode,
    radius: f64,
    #[serde(default = "default_show_in_tooltip")]
    show_in_tooltip: bool,
    #[serde(default = "default_target_entities")]
    target_entities: bool,
    #[serde(default = "default_trigger_from_target")]
    trigger_from_target: bool,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum AreaTriggerItemCollisionMode {
    #[serde(rename = "distance_from_collision_box")]
    DistanceFromCollisionBox,
    #[serde(rename = "distance_from_center")]
    DistanceFromCenter,
}
fn default_collision_mode() -> AreaTriggerItemCollisionMode {
    AreaTriggerItemCollisionMode::DistanceFromCollisionBox
}
fn default_show_in_tooltip() -> bool {
    true
}
fn default_target_entities() -> bool {
    true
}
fn default_trigger_from_target() -> bool {
    false
}
