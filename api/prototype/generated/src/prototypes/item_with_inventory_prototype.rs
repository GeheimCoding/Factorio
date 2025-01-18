#[derive(serde::Deserialize)]
pub struct ItemWithInventoryPrototype {
    base_: crate::prototypes::ItemWithLabelPrototype,
    #[serde(default = "default_filter_message_key")]
    filter_message_key: String,
    #[serde(default = "default_filter_mode")]
    filter_mode: ItemWithInventoryPrototypeFilterMode,
    inventory_size: crate::types::ItemStackIndex,
    item_filters: Vec<crate::types::ItemID>,
    item_group_filters: Vec<crate::types::ItemGroupID>,
    item_subgroup_filters: Vec<crate::types::ItemSubGroupID>,
    stack_size: f64,
}
fn default_filter_message_key() -> String {
    String::from("item-limitation.item-not-allowed-in-this-container-item")
}
#[derive(serde::Deserialize)]
pub enum ItemWithInventoryPrototypeFilterMode {
    #[serde(rename = "blacklist")]
    Blacklist,
    #[serde(rename = "whitelist")]
    Whitelist,
}
fn default_filter_mode() -> ItemWithInventoryPrototypeFilterMode {
    ItemWithInventoryPrototypeFilterMode::Whitelist
}
