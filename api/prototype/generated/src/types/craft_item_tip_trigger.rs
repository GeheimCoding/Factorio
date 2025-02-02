#[derive(Debug, serde::Deserialize)]
pub struct CraftItemTipTrigger {
    #[serde(flatten)]
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
    #[serde(rename = "crafting-of-single-item-ordered")]
    CraftingOfSingleItemOrdered,
    #[serde(rename = "crafting-of-multiple-items-ordered")]
    CraftingOfMultipleItemsOrdered,
    #[serde(rename = "crafting-finished")]
    CraftingFinished,
}
