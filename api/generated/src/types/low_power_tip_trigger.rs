#[derive(Debug, serde::Deserialize)]
pub struct LowPowerTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
