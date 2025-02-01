#[derive(Debug, serde::Deserialize)]
pub enum SelectionModeFlags {
    #[serde(untagged)]
    SelectionModeFlagsVariants(SelectionModeFlagsVariants),
    #[serde(untagged)]
    VecSelectionModeFlagsVariants(Vec<SelectionModeFlagsVariants>),
}
#[derive(Debug, serde::Deserialize)]
pub enum SelectionModeFlagsVariants {
    #[serde(rename = "blueprint")]
    Blueprint,
    #[serde(rename = "deconstruct")]
    Deconstruct,
    #[serde(rename = "cancel-deconstruct")]
    CancelDeconstruct,
    #[serde(rename = "items")]
    Items,
    #[serde(rename = "trees")]
    Trees,
    #[serde(rename = "buildable-type")]
    BuildableType,
    #[serde(rename = "nothing")]
    Nothing,
    #[serde(rename = "items-to-place")]
    ItemsToPlace,
    #[serde(rename = "any-entity")]
    AnyEntity,
    #[serde(rename = "any-tile")]
    AnyTile,
    #[serde(rename = "same-force")]
    SameForce,
    #[serde(rename = "not-same-force")]
    NotSameForce,
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "enemy")]
    Enemy,
    #[serde(rename = "upgrade")]
    Upgrade,
    #[serde(rename = "cancel-upgrade")]
    CancelUpgrade,
    #[serde(rename = "downgrade")]
    Downgrade,
    #[serde(rename = "entity-with-health")]
    EntityWithHealth,
    #[serde(rename = "is-military-target")]
    IsMilitaryTarget,
    #[serde(rename = "entity-with-owner")]
    EntityWithOwner,
    #[serde(rename = "avoid-rolling-stock")]
    AvoidRollingStock,
    #[serde(rename = "avoid-vehicle")]
    AvoidVehicle,
    #[serde(rename = "controllable")]
    Controllable,
    #[serde(rename = "controllable-add")]
    ControllableAdd,
    #[serde(rename = "controllable-remove")]
    ControllableRemove,
    #[serde(rename = "entity-ghost")]
    EntityGhost,
    #[serde(rename = "tile-ghost")]
    TileGhost,
}
