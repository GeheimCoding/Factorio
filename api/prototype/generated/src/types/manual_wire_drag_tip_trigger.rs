#[derive(serde::Deserialize)]
pub struct ManualWireDragTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    source: crate::types::EntityID,
    target: crate::types::EntityID,
    #[serde(rename = "type")]
    type_: String,
    wire_type: ManualWireDragTipTriggerWireType,
}
fn default_match_type_only() -> bool {
    false
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
