pub type EntityPrototypeFlags = Vec<EntityPrototypeFlagsVariants>;
#[derive(Debug, serde::Deserialize)]
pub enum EntityPrototypeFlagsVariants {
    #[serde(rename = "not_rotatable")]
    NotRotatable,
    #[serde(rename = "placeable_neutral")]
    PlaceableNeutral,
    #[serde(rename = "placeable_player")]
    PlaceablePlayer,
    #[serde(rename = "placeable_enemy")]
    PlaceableEnemy,
    #[serde(rename = "placeable_off_grid")]
    PlaceableOffGrid,
    #[serde(rename = "player_creation")]
    PlayerCreation,
    #[serde(rename = "building_direction_8way")]
    BuildingDirection8Way,
    #[serde(rename = "filter_directions")]
    FilterDirections,
    #[serde(rename = "get_by_unit_number")]
    GetByUnitNumber,
    #[serde(rename = "breaths_air")]
    BreathsAir,
    #[serde(rename = "not_repairable")]
    NotRepairable,
    #[serde(rename = "not_on_map")]
    NotOnMap,
    #[serde(rename = "not_deconstructable")]
    NotDeconstructable,
    #[serde(rename = "not_blueprintable")]
    NotBlueprintable,
    #[serde(rename = "hide_alt_info")]
    HideAltInfo,
    #[serde(rename = "no_gap_fill_while_building")]
    NoGapFillWhileBuilding,
    #[serde(rename = "not_flammable")]
    NotFlammable,
    #[serde(rename = "no_automated_item_removal")]
    NoAutomatedItemRemoval,
    #[serde(rename = "no_automated_item_insertion")]
    NoAutomatedItemInsertion,
    #[serde(rename = "no_copy_paste")]
    NoCopyPaste,
    #[serde(rename = "not_selectable_in_game")]
    NotSelectableInGame,
    #[serde(rename = "not_upgradable")]
    NotUpgradable,
    #[serde(rename = "not_in_kill_statistics")]
    NotInKillStatistics,
    #[serde(rename = "building_direction_16way")]
    BuildingDirection16Way,
    #[serde(rename = "snap_to_rail_support_spot")]
    SnapToRailSupportSpot,
    #[serde(rename = "not_in_made_in")]
    NotInMadeIn,
}
