pub struct CreateStickerTriggerEffectItem {
    base_: crate::types::TriggerEffectItem,
    show_in_tooltip: bool,
    sticker: crate::types::EntityID,
    trigger_created_entity: bool,
    type_: String,
}
