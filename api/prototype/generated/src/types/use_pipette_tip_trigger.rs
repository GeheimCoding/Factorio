#[derive(serde::Deserialize)]
pub struct UsePipetteTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
