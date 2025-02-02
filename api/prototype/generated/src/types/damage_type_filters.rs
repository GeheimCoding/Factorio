#[derive(Debug, serde::Deserialize)]
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
    VecDamageTypeID(crate::vec::Vec<crate::types::DamageTypeID>),
}
#[derive(Debug, serde::Deserialize)]
pub enum DamageTypeFiltersTypes {
    #[serde(untagged)]
    DamageTypeID(crate::types::DamageTypeID),
    #[serde(untagged)]
    VecDamageTypeID(crate::vec::Vec<crate::types::DamageTypeID>),
}
fn default_whitelist() -> bool {
    false
}
