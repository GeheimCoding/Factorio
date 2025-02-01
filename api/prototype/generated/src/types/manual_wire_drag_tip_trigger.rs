#[derive(Debug, serde::Deserialize)]
pub struct ManualWireDragTipTrigger {
    base_: crate::types::CountBasedTipTrigger,
    #[serde(default = "default_match_type_only")]
    match_type_only: bool,
    source: Option<crate::types::EntityID>,
    target: Option<crate::types::EntityID>,
    wire_type: Option<ManualWireDragTipTriggerWireType>,
}
fn default_match_type_only() -> bool {
    false
}
#[derive(Debug, serde::Deserialize)]
pub enum ManualWireDragTipTriggerWireType {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "copper")]
    Copper,
}
