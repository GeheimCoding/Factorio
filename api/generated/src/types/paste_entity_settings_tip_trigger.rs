#[derive(Debug, serde::Deserialize)]
pub struct PasteEntitySettingsTipTrigger {
    #[serde(flatten)]
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    source: Option<crate::types::EntityID>,
    target: Option<crate::types::EntityID>,
}
fn default_match_type_only() -> bool {
    false
}
