#[derive(Debug, serde::Deserialize)]
pub struct ApplyStarterPackTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
