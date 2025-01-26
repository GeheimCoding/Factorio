#[derive(Debug, serde::Deserialize)]
pub struct LowPowerTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
