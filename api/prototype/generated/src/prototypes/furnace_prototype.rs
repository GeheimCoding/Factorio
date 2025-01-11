pub struct FurnacePrototype {
    base_: crate::prototypes::CraftingMachinePrototype,
    cant_insert_at_source_message_key: String,
    custom_input_slot_tooltip_key: String,
    result_inventory_size: crate::types::ItemStackIndex,
    source_inventory_size: crate::types::ItemStackIndex,
}
