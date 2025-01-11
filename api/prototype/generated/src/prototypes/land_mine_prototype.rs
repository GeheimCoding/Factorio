pub struct LandMinePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    action: crate::types::Trigger,
    ammo_category: crate::types::AmmoCategoryID,
    force_die_on_attack: bool,
    is_military_target: bool,
    picture_safe: crate::types::Sprite,
    picture_set: crate::types::Sprite,
    picture_set_enemy: crate::types::Sprite,
    timeout: u32,
    trigger_collision_mask: crate::types::CollisionMaskConnector,
    trigger_force: crate::types::ForceCondition,
    trigger_radius: f64,
}
