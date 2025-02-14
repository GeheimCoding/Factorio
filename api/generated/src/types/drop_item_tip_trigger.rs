#[derive(Debug, serde::Deserialize)]
pub struct DropItemTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_drop_into_entity")]
    drop_into_entity: bool,
}
fn default_drop_into_entity() -> bool {
    false
}
