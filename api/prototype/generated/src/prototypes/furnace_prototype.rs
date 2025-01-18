#[derive(serde::Deserialize)]
pub struct FurnacePrototype {
    base_: crate::prototypes::CraftingMachinePrototype,
    #[serde(default = "default_cant_insert_at_source_message_key")]
    cant_insert_at_source_message_key: String,
    #[serde(default = "default_custom_input_slot_tooltip_key")]
    custom_input_slot_tooltip_key: String,
    result_inventory_size: crate::types::ItemStackIndex,
    source_inventory_size: crate::types::ItemStackIndex,
}
fn default_cant_insert_at_source_message_key() -> String {
    String::from("inventory-restriction.cant-be-smelted")
}
fn default_custom_input_slot_tooltip_key() -> String {
    String::from("")
}
