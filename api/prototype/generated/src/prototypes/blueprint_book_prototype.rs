#[derive(Debug, serde::Deserialize)]
pub struct BlueprintBookPrototype {
    base_: crate::prototypes::ItemWithInventoryPrototype,
    #[serde(default = "default_draw_label_for_cursor_render")]
    draw_label_for_cursor_render: bool,
    inventory_size: BlueprintBookPrototypeInventorySize,
    stack_size: f64,
}
fn default_draw_label_for_cursor_render() -> bool {
    true
}
#[derive(Debug, serde::Deserialize)]
pub enum BlueprintBookPrototypeInventorySize {
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(untagged)]
    ItemStackIndex(crate::types::ItemStackIndex),
}
