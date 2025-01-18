#[derive(serde::Deserialize)]
pub struct ToggleRailLayerTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(rename = "type")]
    type_: String,
}
