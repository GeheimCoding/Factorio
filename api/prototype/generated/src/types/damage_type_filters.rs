#[derive(serde::Deserialize)]
pub enum DamageTypeFilters {
    #[serde(untagged)]
    DamageTypeFilters {
        types: DamageTypeFiltersTypes,
        #[serde(default = "default_whitelist")]
        whitelist: bool,
    },
    #[serde(untagged)]
    DamageTypeID(crate::types::DamageTypeID),
    #[serde(untagged)]
    VecDamageTypeID(Vec<crate::types::DamageTypeID>),
}
#[derive(serde::Deserialize)]
pub enum DamageTypeFiltersTypes {
    #[serde(untagged)]
    DamageTypeID(crate::types::DamageTypeID),
    #[serde(untagged)]
    VecDamageTypeID(Vec<crate::types::DamageTypeID>),
}
fn default_whitelist() -> bool {
    false
}
