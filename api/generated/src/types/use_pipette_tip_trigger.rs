#[derive(Debug, serde::Deserialize)]
pub struct UsePipetteTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
