pub struct CraftItemTipTrigger {
    consecutive: bool,
    event_type: CraftItemTipTriggerEventType,
    item: crate::types::ItemID,
    type_: String,
}
pub enum CraftItemTipTriggerEventType {
    CraftingOfSingleItemOrdered,
    CraftingOfMultipleItemsOrdered,
    CraftingFinished,
}
