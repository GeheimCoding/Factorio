pub enum DamageTypeFilters {
    DamageTypeFilters {
        types: DamageTypeFiltersTypes,
        whitelist: bool,
    },
    DamageTypeID(DamageTypeID),
    VecDamageTypeID(Vec<DamageTypeID>),
}
pub enum DamageTypeFiltersTypes {
    DamageTypeID(DamageTypeID),
    VecDamageTypeID(Vec<DamageTypeID>),
}
