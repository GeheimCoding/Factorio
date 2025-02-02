#[derive(Debug, serde::Deserialize)]
pub struct ApplyStarterPackTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
