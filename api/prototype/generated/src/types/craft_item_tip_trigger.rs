#[derive(serde::Deserialize)]
pub struct CraftItemTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    consecutive: bool,
    event_type: CraftItemTipTriggerEventType,
    item: crate::types::ItemID,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(serde::Deserialize)]
pub enum CraftItemTipTriggerEventType {
    #[serde(rename = "crafting_of_single_item_ordered")]
    CraftingOfSingleItemOrdered,
    #[serde(rename = "crafting_of_multiple_items_ordered")]
    CraftingOfMultipleItemsOrdered,
    #[serde(rename = "crafting_finished")]
    CraftingFinished,
}
