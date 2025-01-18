#[derive(serde::Deserialize)]
pub struct UseConfirmTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
