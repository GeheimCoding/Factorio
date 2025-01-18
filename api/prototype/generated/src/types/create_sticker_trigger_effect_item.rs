#[derive(serde::Deserialize)]
pub struct CreateStickerTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    show_in_tooltip: bool,
    sticker: crate::types::EntityID,
    trigger_created_entity: bool,
    #[serde(rename = "type")]
    type_: String,
}
