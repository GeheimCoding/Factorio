pub type EntityPrototypeFlags = Vec<EntityPrototypeFlagsVariants>;
#[derive(Debug, serde::Deserialize)]
pub enum EntityPrototypeFlagsVariants {
    #[serde(rename = "not-rotatable")]
    NotRotatable,
    #[serde(rename = "placeable-neutral")]
    PlaceableNeutral,
    #[serde(rename = "placeable-player")]
    PlaceablePlayer,
    #[serde(rename = "placeable-enemy")]
    PlaceableEnemy,
    #[serde(rename = "placeable-off-grid")]
    PlaceableOffGrid,
    #[serde(rename = "player-creation")]
    PlayerCreation,
    #[serde(rename = "building-direction-8-way")]
    BuildingDirection8Way,
    #[serde(rename = "filter-directions")]
    FilterDirections,
    #[serde(rename = "get-by-unit-number")]
    GetByUnitNumber,
    #[serde(rename = "breaths-air")]
    BreathsAir,
    #[serde(rename = "not-repairable")]
    NotRepairable,
    #[serde(rename = "not-on-map")]
    NotOnMap,
    #[serde(rename = "not-deconstructable")]
    NotDeconstructable,
    #[serde(rename = "not-blueprintable")]
    NotBlueprintable,
    #[serde(rename = "hide-alt-info")]
    HideAltInfo,
    #[serde(rename = "no-gap-fill-while-building")]
    NoGapFillWhileBuilding,
    #[serde(rename = "not-flammable")]
    NotFlammable,
    #[serde(rename = "no-automated-item-removal")]
    NoAutomatedItemRemoval,
    #[serde(rename = "no-automated-item-insertion")]
    NoAutomatedItemInsertion,
    #[serde(rename = "no-copy-paste")]
    NoCopyPaste,
    #[serde(rename = "not-selectable-in-game")]
    NotSelectableInGame,
    #[serde(rename = "not-upgradable")]
    NotUpgradable,
    #[serde(rename = "not-in-kill-statistics")]
    NotInKillStatistics,
    #[serde(rename = "building-direction-16-way")]
    BuildingDirection16Way,
    #[serde(rename = "snap-to-rail-support-spot")]
    SnapToRailSupportSpot,
    #[serde(rename = "not-in-made-in")]
    NotInMadeIn,
}
