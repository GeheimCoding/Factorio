pub struct BlueprintBookPrototype {
    draw_label_for_cursor_render: bool,
    inventory_size: BlueprintBookPrototypeInventorySize,
    stack_size: String,
}
pub enum BlueprintBookPrototypeInventorySize {
    ItemStackIndex(crate::types::ItemStackIndex),
    Dynamic,
}