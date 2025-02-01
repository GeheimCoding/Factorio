#[derive(Debug, serde::Deserialize)]
pub struct CreateStickerTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    #[serde(default = "default_show_in_tooltip")]
    show_in_tooltip: bool,
    sticker: crate::types::EntityID,
    #[serde(default = "default_trigger_created_entity")]
    trigger_created_entity: bool,
}
fn default_show_in_tooltip() -> bool {
    false
}
fn default_trigger_created_entity() -> bool {
    false
}
