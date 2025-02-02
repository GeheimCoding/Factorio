#[derive(Debug, serde::Deserialize)]
pub struct UseConfirmTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
