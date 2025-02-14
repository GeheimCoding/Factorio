#[derive(Debug, serde::Deserialize)]
pub struct GeneratingPowerTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
