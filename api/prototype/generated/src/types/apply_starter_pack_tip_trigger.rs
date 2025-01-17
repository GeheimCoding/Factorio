#[derive(serde::Deserialize)]
pub struct ApplyStarterPackTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
