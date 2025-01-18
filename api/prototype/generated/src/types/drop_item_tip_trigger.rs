#[derive(serde::Deserialize)]
pub struct DropItemTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_drop_into_entity")]
    drop_into_entity: bool,
    #[serde(rename = "type")]
    type_: String,
}
fn default_drop_into_entity() -> bool {
    false
}
