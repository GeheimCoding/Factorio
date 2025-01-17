pub struct ItemWithInventoryPrototype {
    base_: crate::prototypes::ItemWithLabelPrototype,
    filter_message_key: String,
    filter_mode: ItemWithInventoryPrototypeFilterMode,
    inventory_size: crate::types::ItemStackIndex,
    item_filters: Vec<crate::types::ItemID>,
    item_group_filters: Vec<crate::types::ItemGroupID>,
    item_subgroup_filters: Vec<crate::types::ItemSubGroupID>,
    stack_size: String,
}
#[derive(serde::Deserialize)]
pub enum ItemWithInventoryPrototypeFilterMode {
    #[serde(rename = "blacklist")]
    Blacklist,
    #[serde(rename = "whitelist")]
    Whitelist,
}
