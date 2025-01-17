#[derive(serde::Deserialize)]
pub struct GeneratingPowerTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
