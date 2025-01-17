#[derive(serde::Deserialize)]
pub struct LowPowerTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
