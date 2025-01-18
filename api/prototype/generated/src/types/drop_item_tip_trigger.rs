#[derive(serde::Deserialize)]
pub struct DropItemTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    drop_into_entity: bool,
    #[serde(rename = "type")]
    type_: String,
}
