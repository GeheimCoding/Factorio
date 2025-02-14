#[derive(Debug, serde::Deserialize)]
pub struct ToggleRailLayerTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
}
