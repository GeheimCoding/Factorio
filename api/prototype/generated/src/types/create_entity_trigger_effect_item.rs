pub struct CreateEntityTriggerEffectItem {
    as_enemy: bool,
    check_buildability: bool,
    entity_name: crate::types::EntityID,
    find_non_colliding_position: bool,
    ignore_no_enemies_mode: bool,
    non_colliding_fail_result: crate::types::Trigger,
    non_colliding_search_precision: f64,
    non_colliding_search_radius: f64,
    offset_deviation: crate::types::BoundingBox,
    offsets: Vec<crate::types::Vector>,
    protected: bool,
    show_in_tooltip: bool,
    tile_collision_mask: crate::types::CollisionMaskConnector,
    trigger_created_entity: bool,
    type_: CreateEntity,
}
