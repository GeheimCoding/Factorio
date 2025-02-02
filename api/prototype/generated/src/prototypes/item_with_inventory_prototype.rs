#[derive(Debug, serde::Deserialize)]
pub struct ItemWithInventoryPrototype {
    #[serde(flatten)]
    base_: crate::prototypes::ItemWithLabelPrototype,
    #[serde(default = "default_filter_message_key")]
    filter_message_key: String,
    #[serde(default = "default_filter_mode")]
    filter_mode: ItemWithInventoryPrototypeFilterMode,
    item_filters: Option<crate::vec::Vec<crate::types::ItemID>>,
    item_group_filters: Option<crate::vec::Vec<crate::types::ItemGroupID>>,
    item_subgroup_filters: Option<crate::vec::Vec<crate::types::ItemSubGroupID>>,
}
fn default_filter_message_key() -> String {
    String::from("item-limitation.item-not-allowed-in-this-container-item")
}
#[derive(Debug, serde::Deserialize)]
pub enum ItemWithInventoryPrototypeFilterMode {
    #[serde(rename = "blacklist")]
    Blacklist,
    #[serde(rename = "whitelist")]
    Whitelist,
}
fn default_filter_mode() -> ItemWithInventoryPrototypeFilterMode {
    ItemWithInventoryPrototypeFilterMode::Whitelist
}
