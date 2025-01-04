pub enum DamageTypeFilters {
    DamageTypeFilters {
        types: DamageTypeFiltersTypes,
        whitelist: bool,
    },
    DamageTypeID(crate::types::DamageTypeID),
    VecDamageTypeID(Vec<crate::types::DamageTypeID>),
}
pub enum DamageTypeFiltersTypes {
    DamageTypeID(crate::types::DamageTypeID),
    VecDamageTypeID(Vec<crate::types::DamageTypeID>),
}
