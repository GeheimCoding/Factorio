#[derive(serde::Deserialize)]
pub struct BlueprintBookPrototype {
    base_: crate::prototypes::ItemWithInventoryPrototype,
    draw_label_for_cursor_render: bool,
    inventory_size: BlueprintBookPrototypeInventorySize,
    stack_size: String,
}
#[derive(serde::Deserialize)]
pub enum BlueprintBookPrototypeInventorySize {
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(untagged)]
    ItemStackIndex(crate::types::ItemStackIndex),
}
