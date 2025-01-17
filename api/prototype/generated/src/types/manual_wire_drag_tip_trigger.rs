#[derive(serde::Deserialize)]
pub struct ManualWireDragTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    match_type_only: bool,
    source: crate::types::EntityID,
    target: crate::types::EntityID,
    type_: String,
    wire_type: ManualWireDragTipTriggerWireType,
}
#[derive(serde::Deserialize)]
pub enum ManualWireDragTipTriggerWireType {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "copper")]
    Copper,
}
