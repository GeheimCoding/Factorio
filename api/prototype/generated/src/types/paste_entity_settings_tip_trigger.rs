#[derive(serde::Deserialize)]
pub struct PasteEntitySettingsTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    source: crate::types::EntityID,
    target: crate::types::EntityID,
    #[serde(rename = "type")]
    type_: String,
}
fn default_match_type_only() -> bool {
    false
}
