#[derive(serde::Deserialize)]
pub struct PasteEntitySettingsTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    match_type_only: bool,
    source: crate::types::EntityID,
    target: crate::types::EntityID,
    type_: String,
}
