#[derive(serde::Deserialize)]
pub struct ToggleRailLayerTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    type_: String,
}
