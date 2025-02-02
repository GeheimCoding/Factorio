#[derive(Debug, serde::Deserialize)]
pub struct CreateEntityTriggerEffectItem {
    #[serde(flatten)]
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_as_enemy")]
    as_enemy: bool,
    #[serde(default = "default_check_buildability")]
    check_buildability: bool,
    entity_name: crate::types::EntityID,
    #[serde(default = "default_find_non_colliding_position")]
    find_non_colliding_position: bool,
    #[serde(default = "default_ignore_no_enemies_mode")]
    ignore_no_enemies_mode: bool,
    non_colliding_fail_result: Option<crate::types::Trigger>,
    #[serde(default = "default_non_colliding_search_precision")]
    non_colliding_search_precision: f64,
    #[serde(default = "default_non_colliding_search_radius")]
    non_colliding_search_radius: f64,
    offset_deviation: Option<crate::types::BoundingBox>,
    offsets: Option<Vec<crate::types::Vector>>,
    #[serde(default = "default_protected")]
    protected: bool,
    #[serde(default = "default_show_in_tooltip")]
    show_in_tooltip: bool,
    tile_collision_mask: Option<crate::types::CollisionMaskConnector>,
    #[serde(default = "default_trigger_created_entity")]
    trigger_created_entity: bool,
}
fn default_as_enemy() -> bool {
    false
}
fn default_check_buildability() -> bool {
    false
}
fn default_find_non_colliding_position() -> bool {
    false
}
fn default_ignore_no_enemies_mode() -> bool {
    false
}
fn default_non_colliding_search_precision() -> f64 {
    0.2
}
fn default_non_colliding_search_radius() -> f64 {
    5.0
}
fn default_protected() -> bool {
    false
}
fn default_show_in_tooltip() -> bool {
    false
}
fn default_trigger_created_entity() -> bool {
    false
}
