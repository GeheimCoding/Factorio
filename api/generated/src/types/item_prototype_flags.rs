pub type ItemPrototypeFlags = crate::vec::Vec<ItemPrototypeFlagsVariants>;
#[derive(Debug, serde::Deserialize)]
pub enum ItemPrototypeFlagsVariants {
    #[serde(rename = "draw-logistic-overlay")]
    DrawLogisticOverlay,
    #[serde(rename = "excluded-from-trash-unrequested")]
    ExcludedFromTrashUnrequested,
    #[serde(rename = "always-show")]
    AlwaysShow,
    #[serde(rename = "hide-from-bonus-gui")]
    HideFromBonusGui,
    #[serde(rename = "hide-from-fuel-tooltip")]
    HideFromFuelTooltip,
    #[serde(rename = "not-stackable")]
    NotStackable,
    #[serde(rename = "primary-place-result")]
    PrimaryPlaceResult,
    #[serde(rename = "mod-openable")]
    ModOpenable,
    #[serde(rename = "only-in-cursor")]
    OnlyInCursor,
    #[serde(rename = "spawnable")]
    Spawnable,
    #[serde(rename = "spoil-result")]
    SpoilResult,
    #[serde(rename = "ignore-spoil-time-modifier")]
    IgnoreSpoilTimeModifier,
}
