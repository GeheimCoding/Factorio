#[derive(serde::Deserialize)]
pub enum SelectionModeFlags {
    #[serde(untagged)]
    SelectionModeFlagsVariants(SelectionModeFlagsVariants),
    #[serde(untagged)]
    VecSelectionModeFlagsVariants(Vec<SelectionModeFlagsVariants>),
}
#[derive(serde::Deserialize)]
pub enum SelectionModeFlagsVariants {
    #[serde(rename = "blueprint")]
    Blueprint,
    #[serde(rename = "deconstruct")]
    Deconstruct,
    #[serde(rename = "cancel_deconstruct")]
    CancelDeconstruct,
    #[serde(rename = "items")]
    Items,
    #[serde(rename = "trees")]
    Trees,
    #[serde(rename = "buildable_type")]
    BuildableType,
    #[serde(rename = "nothing")]
    Nothing,
    #[serde(rename = "items_to_place")]
    ItemsToPlace,
    #[serde(rename = "any_entity")]
    AnyEntity,
    #[serde(rename = "any_tile")]
    AnyTile,
    #[serde(rename = "same_force")]
    SameForce,
    #[serde(rename = "not_same_force")]
    NotSameForce,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "upgrade")]
    Upgrade,
    #[serde(rename = "cancel_upgrade")]
    CancelUpgrade,
    #[serde(rename = "downgrade")]
    Downgrade,
    #[serde(rename = "entity_with_health")]
    EntityWithHealth,
    #[serde(rename = "is_military_target")]
    IsMilitaryTarget,
    #[serde(rename = "entity_with_owner")]
    EntityWithOwner,
    #[serde(rename = "avoid_rolling_stock")]
    AvoidRollingStock,
    #[serde(rename = "avoid_vehicle")]
    AvoidVehicle,
    #[serde(rename = "controllable")]
    Controllable,
    #[serde(rename = "controllable_add")]
    ControllableAdd,
    #[serde(rename = "controllable_remove")]
    ControllableRemove,
    #[serde(rename = "entity_ghost")]
    EntityGhost,
    #[serde(rename = "tile_ghost")]
    TileGhost,
}
