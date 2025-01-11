pub struct CraftItemTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
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
