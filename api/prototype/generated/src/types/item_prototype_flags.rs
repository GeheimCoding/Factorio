pub type ItemPrototypeFlags = Vec<ItemPrototypeFlagsVariants>;
pub enum ItemPrototypeFlagsVariants {
    DrawLogisticOverlay,
    ExcludedFromTrashUnrequested,
    AlwaysShow,
    HideFromBonusGui,
    HideFromFuelTooltip,
    NotStackable,
    PrimaryPlaceResult,
    ModOpenable,
    OnlyInCursor,
    Spawnable,
    SpoilResult,
    IgnoreSpoilTimeModifier,
}
