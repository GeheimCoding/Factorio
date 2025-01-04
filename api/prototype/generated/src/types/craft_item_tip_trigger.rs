pub struct CraftItemTipTrigger {
    consecutive: bool,
    event_type: CraftItemTipTriggerEventType,
    item: crate::types::ItemID,
    type_: CraftItem,
}
pub enum CraftItemTipTriggerEventType {
    CraftingOfSingleItemOrdered,
    CraftingOfMultipleItemsOrdered,
    CraftingFinished,
}
