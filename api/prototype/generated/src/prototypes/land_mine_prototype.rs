#[derive(serde::Deserialize)]
pub struct LandMinePrototype {
    base_: crate::prototypes::EntityWithOwnerPrototype,
    action: Option<crate::types::Trigger>,
    ammo_category: Option<crate::types::AmmoCategoryID>,
    #[serde(default = "default_force_die_on_attack")]
    force_die_on_attack: bool,
    #[serde(default = "default_is_military_target")]
    is_military_target: bool,
    picture_safe: Option<crate::types::Sprite>,
    picture_set: Option<crate::types::Sprite>,
    picture_set_enemy: Option<crate::types::Sprite>,
    #[serde(default = "default_timeout")]
    timeout: u32,
    // default: Value of UtilityConstants::building_collision_mask
    trigger_collision_mask: Option<crate::types::CollisionMaskConnector>,
    #[serde(default = "default_trigger_force")]
    trigger_force: crate::types::ForceCondition,
    trigger_radius: f64,
}
fn default_force_die_on_attack() -> bool {
    true
}
fn default_is_military_target() -> bool {
    true
}
fn default_timeout() -> u32 {
    120
}
fn default_trigger_force() -> crate::types::ForceCondition {
    crate::types::ForceCondition::Enemy
}
