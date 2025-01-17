#[derive(serde::Deserialize)]
pub struct UseConfirmTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
