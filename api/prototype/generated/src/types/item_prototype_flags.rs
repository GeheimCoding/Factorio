pub type ItemPrototypeFlags = Vec<ItemPrototypeFlagsVariants>;
#[derive(serde::Deserialize)]
pub enum ItemPrototypeFlagsVariants {
    #[serde(rename = "draw_logistic_overlay")]
    DrawLogisticOverlay,
    #[serde(rename = "excluded_from_trash_unrequested")]
    ExcludedFromTrashUnrequested,
    #[serde(rename = "always_show")]
    AlwaysShow,
    #[serde(rename = "hide_from_bonus_gui")]
    HideFromBonusGui,
    #[serde(rename = "hide_from_fuel_tooltip")]
    HideFromFuelTooltip,
    #[serde(rename = "not_stackable")]
    NotStackable,
    #[serde(rename = "primary_place_result")]
    PrimaryPlaceResult,
    #[serde(rename = "mod_openable")]
    ModOpenable,
    #[serde(rename = "only_in_cursor")]
    OnlyInCursor,
    #[serde(rename = "spawnable")]
    Spawnable,
    #[serde(rename = "spoil_result")]
    SpoilResult,
    #[serde(rename = "ignore_spoil_time_modifier")]
    IgnoreSpoilTimeModifier,
}
