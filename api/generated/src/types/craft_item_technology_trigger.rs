#[derive(Debug, serde::Deserialize)]
pub struct CraftItemTechnologyTrigger {
    #[serde(default = "default_count")]
    count: crate::types::ItemCountType,
    item: crate::types::ItemIDFilter,
}
fn default_count() -> crate::types::ItemCountType {
    1
}
