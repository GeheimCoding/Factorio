pub struct CraftItemTipTrigger {
    consecutive: bool,
    event_type: CraftItemTipTriggerEventType,
    item: ItemID,
    type_: CraftItem,
}
pub enum CraftItemTipTriggerEventType {
    CraftingOfSingleItemOrdered,
    CraftingOfMultipleItemsOrdered,
    CraftingFinished,
}
