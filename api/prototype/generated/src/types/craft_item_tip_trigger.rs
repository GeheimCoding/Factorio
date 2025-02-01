#[derive(Debug, serde::Deserialize)]
pub struct CraftItemTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_consecutive")]
    consecutive: bool,
    event_type: CraftItemTipTriggerEventType,
    item: Option<crate::types::ItemID>,
}
fn default_consecutive() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum CraftItemTipTriggerEventType {
    #[serde(rename = "crafting_of_single_item_ordered")]
    CraftingOfSingleItemOrdered,
    #[serde(rename = "crafting_of_multiple_items_ordered")]
    CraftingOfMultipleItemsOrdered,
    #[serde(rename = "crafting_finished")]
    CraftingFinished,
}
